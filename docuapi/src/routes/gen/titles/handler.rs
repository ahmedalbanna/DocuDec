use std::sync::Arc;

use actix_web::{
    http::header,
    web::{self, head, Json},
    HttpRequest, HttpResponse,
};
use docuai::DocuRequest;
use google_generative_ai_rs::v1::api::Client;
use tracing::{instrument, trace};

use super::super::error::GenError;
use crate::routes::gen::generate_report;

use super::super::validators::validate_title_content;
use super::request::TitleRequest;

/// # Endpoint Documentation: Project Title Handling
///
/// # Endpoint
/// ```
///     /ge/titles
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
/// If the request is successful, the response will return alternative titles as the
/// AlternativeTitles struct
///
///
/// For details on the `AlternativeTitles`, refer to the [`AlternativeTitles`](../../../../../docuai/docu_requests/schemas/alter_titles/struct.AlternativeTitles.html) struct.
///
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
/// If there’s an issue with the provided JSON:
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
/// ### 4. **Bearer Token Issues**
///
/// - If the `Authorization` header is missing the `Bearer` prefix:
///
/// ```rust
/// GenError::NoBearerInHeader => (400, "Unable to parse the Bearer token from the header")
/// ```
///
/// - If the header contains non-ASCII characters:
///
/// ```rust
/// GenError::HeaderIsNotInASCII => (400, "The header contains non-ASCII characters")
/// ```
///
/// ### 5. **Database Errors**
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
/// ## Panics
///
/// This endpoint will panic in the following rare scenarios:
///
/// 1. If it fails to retrieve the ``AUTHORIZATION`` header, which is practically impossible.
/// 2. If the `user_id` cannot be converted into a `Uuid`.
/// """
#[instrument(skip(model), fields(title = payload.title))]
#[actix_web::post("/titles")]
pub async fn generate_titles(
    model: web::Data<Arc<Client>>,
    req: HttpRequest,
    payload: Json<TitleRequest>,
) -> Result<HttpResponse, GenError> {
    let jwt_token = req
        .headers()
        .get(header::AUTHORIZATION)
        .expect("the endpoint is not accessable without the header, so we're fine")
        .to_str()
        .map_err(|_| GenError::HeaderIsNotInASCII)?
        .strip_prefix("Bearer ")
        .ok_or(GenError::NoBearerInHeader)?;

    docubase::utils::validate_jwt(jwt_token)?;

    validate_title_content(&payload.title)?;

    trace!("Generating titles | Input: '{}' ", payload.title);
    let alternative_titles = generate_report(
        model.as_ref().as_ref(),
        &DocuRequest::new().with_titles(&payload.title),
    )
    .await?;

    trace!("Generated titles: {:#?}", alternative_titles);

    Ok(HttpResponse::Ok().json(alternative_titles))
}
