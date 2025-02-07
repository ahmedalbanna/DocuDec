use std::str::FromStr;
use std::sync::Arc;

use actix_web::web::{self, Json};
use actix_web::{HttpRequest, HttpResponse};
use docuai::docu_requests::schemas::ProjectInfo;
use docuai::DocuRequest;
use docubase::DocuBase;
use google_generative_ai_rs::v1::api::Client;
use tracing::{instrument, trace};
use uuid::Uuid;

use crate::routes::gen::generate_report;

use super::super::error::GenError;
use super::request::ProjectInfoRequest;

use super::super::validators::validate_title::validate_title_request;

///
/// # Request
/// ```json
/// {
///     "title": "..."
/// }
/// ```
///
/// # SUCCESS
/// ```json
/// {
///     // save this somewhere
///     "project_id": "6efac080-55e7-495f-a6a6-e9641440034e",
///     "content": { // ProjectInfo }
/// }
///
/// ```
/// [`ProjectInfo`](../../../../../docuai/docu_requests/schemas/project_info/struct.ProjectInfo.html)
///
/// # Errors
///
/// ```rust
/// // if there is something wrong with the ai api
/// GenError::ApiRequestError => (500 , "Internal AI error: { String }"),
///
/// // if there is something wrong with the provided json
/// GenError::SyntaxError => (400, "Syntax error: { String }")
///
/// // if the validation failed, it will return what field is wrong, and what's the error
/// // it might return like this {
///          "title": "Title must be between 30 and 80 characters"
///     }
/// // the rules are min = 30, max = 80
/// // other validation error => [
///     "Title cannot be purely numeric",
///     "Excessive special characters in title",
/// ]
///
/// GenError::ValidationError(...) => (400, "{field}: {error}")
///
/// GenError::DBError => {
///        // something wrong with validating the user jwt
///        DocuBaseError::JwtError(...) => (500, "jwt error: { jsonwebtoken::errors::Error }"),
///
///        // internal database error
///        DocuBaseError::SqlxError => (500, "sqlx error: { sqlx::Error }")
///
///
///        // if for some reason the user have a valid jwt, but the user got deleted
///        DocuBaseError::UserDoesNotExist => (404, "user does not exist"),
///
///        // this will not happen in `project-info` but for other, it might happen
///        DocuBaseError::ProjectDoesNotExist => (404, "project does not exist"),
///
///        // if the user provided a project-id that does not belong to him
///        // again this will not happen in `project-info`
///        DocuBaseError::UserDoesNotOwnProject => (403, "user does not own the project"),
///
///        // this might happen if the user provided a report that he already did before
///        // this also will not happen in `project-info`
///        DocuBaseError::ReportExist => (409, "report (type) already exist"),
///
///        // if the user is trying to generate a project with the exact same title
///        DocuBaseError::ProjectExist => (409, "project already exist"),
///
///
///     }
/// ```
///
/// # Panics
///
/// it will panic if it couldn't get the cookie, which is impossible
///
/// or the user_id is not convertable to a Uuid
#[instrument(skip(model, db), fields(title = payload.title))]
#[actix_web::post("/project-info")]
pub async fn generate_project_info(
    model: web::Data<Arc<Client>>,
    db: web::Data<Arc<DocuBase>>,
    req: HttpRequest,
    payload: Json<ProjectInfoRequest>,
) -> Result<HttpResponse, GenError> {
    let jwt_token = req
        .cookie(crate::utils::constants::JWT_COOKIE)
        .expect("the endpoint is not accessable without the cookie, so we're fine");

    let user_id = docubase::utils::validate_jwt(jwt_token.value()).map(|s| {
        Uuid::from_str(&s).unwrap_or_else(|e| {
            panic!("can't convert the id to a Uuid, original is: {s}, error: {e}")
        })
    })?;

    let project_res = db.create_project(&user_id, &payload.title).await?;

    validate_title_request(&payload)?;

    trace!(">> Generating project info for title: {}", payload.title);

    let project_info =
        generate_report(model.as_ref(), DocuRequest::new().with_info(&payload.title)).await?;

    db.create_report(&project_res.id, &user_id, &project_info)
        .await?;

    let response = serde_json::json!({
        "project_id": project_res.id,
        "content": project_info
    });

    trace!(">> Generated project info details: \n{:#?}", project_info);

    Ok(HttpResponse::Ok().json(response))
}
