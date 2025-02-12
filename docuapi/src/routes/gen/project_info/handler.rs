use std::str::FromStr;
use std::sync::Arc;

use actix_web::http::header;
use actix_web::web::{self, Json};
use actix_web::{HttpRequest, HttpResponse};
use docuai::DocuRequest;
use docubase::DocuBase;
use google_generative_ai_rs::v1::api::Client;
use tracing::{instrument, trace};
use uuid::Uuid;

use crate::routes::gen::generate_report;

use super::super::error::GenError;
use super::request::ProjectInfoRequest;

use super::super::validators::validate_title_content;

/// # Endpoint Documentation: Project Info
///
/// ## Endpoint
/// ```
///     /gen/project-info
/// ```
///
/// ## Request
///
/// ```json
/// {
///     "title": "..."
/// }
/// ```
///
/// ### Headers:
/// You must provide your JWT token inside the `Authorization` header:
///
/// ```
/// Authorization: Bearer JWT_TOKEN_HERE
/// ```
///
/// ## Successful Response
///
/// On success, the response will return a project ID and the project content.
///
/// ```json
/// {
///     "project_id": "6efac080-55e7-495f-a6a6-e9641440034e",
///     "content": { // ProjectInfo }
/// }
/// ```
///  You must save the project_id somewhere to use it later on.
///
/// For details on the `ProjectInfo`, refer to the [`ProjectInfo`](../../../../../docuai/docu_requests/schemas/project_info/struct.ProjectInfo.html) struct.
///
/// ## Error Handling
///
/// ### 1. **API Request Error**
///
/// If there is an issue with the AI API, the server will respond with:
///
/// ```rust
/// GenError::ApiRequestError => (500, "Internal AI error: { String }")
/// ```
///
/// ### 2. **Syntax Error**
///
/// If there’s a problem with the provided JSON:
///
/// ```rust
/// GenError::SyntaxError => (400, "Syntax error: { String }")
/// ```
///
/// ### 3. **Validation Errors**
///
/// If validation fails, the response will contain the specific field and error:
///
/// ```json
/// {
///     "title": "Title must be between 20 and 80 characters"
/// }
/// ```
///
/// #### Validation Rules:
/// - Title: Min length = 20, Max length = 80
/// - Other possible validation errors include:
///     - "Title cannot be purely numeric"
///     - "Excessive special characters in title"
///
/// ```rust
/// GenError::ValidationError(...) => (400, "{field}: {error}")
/// ```
///
/// ### 4. **Database Errors**
///
/// These errors occur due to issues with user authentication or database operations.
///
/// #### JWT Errors:
/// If there is an issue with validating the user’s JWT:
///
/// ```rust
/// DocuBaseError::JwtError(...) => (500, "JWT error: { jsonwebtoken::errors::Error }")
/// ```
///
/// #### Database Errors:
/// For internal database errors:
///
/// ```rust
/// DocuBaseError::SqlxError => (500, "SQLx error: { sqlx::Error }")
/// ```
///
/// #### User-Related Errors:
/// - If the user doesn’t exist:
///
/// ```rust
/// DocuBaseError::UserDoesNotExist => (404, "User does not exist")
/// ```
///
/// - If the user doesn’t own the project (not applicable for this endpoint):
///
/// ```rust
/// DocuBaseError::UserDoesNotOwnProject => (403, "User does not own the project")
/// ```
///
/// - If the report already exists (not applicable for this endpoint):
///
/// ```rust
/// DocuBaseError::ReportExist => (409, "Report (type) already exists")
/// ```
///
/// - If the project with the same title already exists:
///
/// ```rust
/// DocuBaseError::ProjectExist => (409, "Project already exists")
/// ```
///
/// ## Panics
///
/// This endpoint will panic in the following rare scenarios:
///
/// 1. If it fails to retrieve the ``AUTHORIZATION``, which is practically impossible.
/// 2. If the `user_id` cannot be converted into a `Uuid`.
/// ```
#[instrument(skip(model, db), fields(title = payload.title))]
#[actix_web::post("/project-info")]
pub async fn generate_project_info(
    model: web::Data<Arc<Client>>,
    db: web::Data<Arc<DocuBase>>,
    req: HttpRequest,
    payload: Json<ProjectInfoRequest>,
) -> Result<HttpResponse, GenError> {
    let jwt_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .expect("the endpoint is not accessable without the header, so we're fine")
        .to_str()
        .map_err(|_| GenError::HeaderIsNotInASCII)?
        .strip_prefix("Bearer ")
        .ok_or(GenError::NoBearerInHeader)?;

    let user_id = docubase::utils::validate_jwt(jwt_token).map(|s| {
        Uuid::from_str(&s).unwrap_or_else(|e| {
            panic!("can't convert the id to a Uuid, original is: {s}, error: {e}")
        })
    })?;

    validate_title_content(&payload.title)?;

    let project_res = db.create_project(&user_id, &payload.title).await?;

    trace!(">> Generating project info for title: {}", payload.title);

    let project_info = generate_report(
        model.as_ref().as_ref(),
        &DocuRequest::new().with_info(&payload.title),
    )
    .await?;

    db.create_report(&project_res.id, &user_id, &project_info)
        .await?;

    let response = serde_json::json!({
        "project_id": project_res.id,
        "content": project_info
    });

    trace!(">> Generated project info details: \n{:#?}", project_info);

    Ok(HttpResponse::Ok().json(response))
}
