use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docuai::{
    docu_requests::{schemas::AlternativeTitles, DocuRequest},
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use tracing::{error, instrument, trace};

use super::super::error::GenError;
use crate::routes::gen::error::map_ai_error;

use super::super::validators::validate_title::validate_title_request;
use super::request::TitleRequest;

/// # Errors
///
/// possible returnd errors are
///
/// ```rust
/// GenError::AiRequestError(...) => (500, "Internal AI error: Couldn't complete the AI request, error: {...}")
/// GenError::UnknownError => (500, "Unrecoverable error generating titles: {...}"),
///
/// GenError::ValidationError(...) =>
/// // possible validation error are
///
/// [
///     // TODO: we should properly increase the maximum characters from 150
///     "Title must be between 3 and 150 characters", // titles are bleow 3 and over 150
///     "Title cannot be purely numeric", // title is all numbers "1234"
///     "Excessive special characters in title", // title have alot of symbols (e.g "!!!!title!!!!")
///     "Invalid title pattern", // title have common testing words, ["test", "placeholder", "example", "untitled", "new project"]
///     "Incorrect Content-Type",
/// ]
/// ```
#[instrument(skip(model), fields(title = payload.title))]
#[actix_web::post("/titles")]
pub async fn generate_titles(
    model: web::Data<Arc<Client>>,
    payload: Json<TitleRequest>,
) -> Result<HttpResponse, GenError> {
    validate_title_request(&payload)?;

    trace!("Generating titles | Input: '{}' ", payload.title);

    let result = generate_titles_(model.as_ref(), &payload.title).await?;

    trace!("Generated titles: {:#?}", result);

    Ok(HttpResponse::Ok().json(result))
}

async fn generate_titles_(model: &Client, title: &str) -> Result<AlternativeTitles, GenError> {
    match docuai::request::<AlternativeTitles>(
        model,
        DocuRequest::AlterTitles(title.to_string()),
        100,
    )
    .await
    {
        Ok(titles) => Ok(titles),
        Err(error) => {
            error!("Title generation failed: {:?}", error);

            if let DocuError::ApiRequest(ge) = error {
                error!(
                    "Failed to generate titles. Error: {}, Status: {:?}",
                    ge.message, ge.code
                );
                Err(map_ai_error(&ge))
            } else {
                error!("Unrecoverable error generating titles: {}", error);
                Err(GenError::UnknownError)
            }
        }
    }
}
