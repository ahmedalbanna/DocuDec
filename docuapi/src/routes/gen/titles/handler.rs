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

/// # Request
/// ```json
/// {
///     "title": "..."
/// }
/// ```
/// ## You must provide your jwt token inside an ``Authorization`` header
///
/// Authorization: Bearer JWT_TOKEN_HERE
///
///
/// # Success
///
/// {
///     [`AlternativeTitles`](../../../../../docuai/docu_requests/schemas/alter_titles/struct.AlternativeTitles.html)
/// }
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
///          "title": "Title must be between 20 and 80 characters"
///     }
/// // the rules are min = 20, max = 80
/// // other validation error => [
///     "Title cannot be purely numeric",
///     "Excessive special characters in title",
/// ]
///
/// GenError::ValidationError(...) => (400, "{field}: {error}")
///
/// // if there is no ``Bearer `` in the header
/// GenError::NoBearerInHeader => (400, "unable to parse the Bearer prefixed in the header")
///
/// GenError::HeaderIsNotInASCII => (400, "the header contains non ascii character")
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
///
///     }
/// ```
///
/// # Panics
///
/// it will panic if it couldn't get the cookie, which is impossible
///
/// or the user_id is not convertable to a Uuid
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
