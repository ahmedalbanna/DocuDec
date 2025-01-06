use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docuai::{
    docu_requests::{responses::alter_titles::Titles, DocuRequest},
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, error, instrument};

use super::super::error::GenError;
use crate::routes::gen::error::map_ai_error;

use super::super::validators::validate_title::validate_title_request;
use super::request::TitleRequest;

#[instrument(skip(model), fields(title = payload.title))]
#[actix_web::post("/titles")]
pub async fn generate_titles(
    model: web::Data<Arc<Client>>,
    payload: Json<TitleRequest>,
) -> Result<HttpResponse, GenError> {
    // Validate input
    validate_title_request(&payload)?;

    debug!("Generating titles | Input: '{}' ", payload.title);

    // Attempt to generate titles
    let result = generate_titles_(model.as_ref(), &payload.title).await?;

    debug!("Generated titles: {:#?}", result);

    Ok(HttpResponse::Ok().json(result))
}

/// Generate titles with error handling
async fn generate_titles_(model: &Client, title: &str) -> Result<Titles, GenError> {
    match docuai::request::<Titles>(model, DocuRequest::AlterTitles(title.to_string()), 100).await {
        Ok(titles) => Ok(titles),
        Err(error) => {
            error!("Title generation failed: {:?}", error);

            match error {
                DocuError::ApiRequest(ge) => {
                    error!(
                        "Failed to generate titles. Error: {}, Status: {:?}",
                        ge.message, ge.code
                    );
                    Err(map_ai_error(ge))
                }
                _ => {
                    error!("Unrecoverable error generating titles: {}", error);
                    Err(GenError::UnknownError)
                }
            }
        }
    }
}
