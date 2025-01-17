use std::sync::Arc;

use actix_web::web::{self, Json};
use actix_web::HttpResponse;
use docuai::docu_requests::schemas::project_info::ProjectInfo;
use docuai::{DocuError, DocuRequest};
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, error, instrument};

use crate::routes::gen::error::map_ai_error;

use super::super::error::GenError;
use super::request::ProjectInfoRequest;

use super::super::validators::validate_title::validate_title_request;

#[instrument(skip(model), fields(title = payload.title))]
#[actix_web::post("/project-info")]
pub async fn generate_project_info(
    model: web::Data<Arc<Client>>,
    payload: Json<ProjectInfoRequest>,
) -> Result<HttpResponse, GenError> {
    validate_title_request(&payload)?;

    debug!(">> Generating project info for title: {}", payload.title);

    // Generate project info with robust error handling

    let result = generate_project_info_(model.as_ref(), &payload.title).await?;

    debug!(">> Generated project info details: \n{:#?}", result);

    Ok(HttpResponse::Ok().json(result))
}

async fn generate_project_info_(model: &Client, title: &str) -> Result<ProjectInfo, GenError> {
    match docuai::request::<ProjectInfo>(model, DocuRequest::ProjectInfo(title.to_string()), 100)
        .await
    {
        Ok(project_info) => Ok(project_info),

        Err(error) => {
            error!("Project info generation failed: {:?}", error);

            match error {
                DocuError::ApiRequest(ge) => {
                    error!(
                        "AI request failed. Error: {}, Status: {:?}",
                        ge.message, ge.code
                    );

                    Err(map_ai_error(ge))
                }

                _ => {
                    error!("Unrecoverable error in project info generation");

                    Err(GenError::UnknownError)
                }
            }
        }
    }
}
