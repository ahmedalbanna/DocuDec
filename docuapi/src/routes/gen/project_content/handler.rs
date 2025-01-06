use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docuai::{
    docu_requests::{
        responses::{project_content::ProjectContent, project_info::ProjectInfo},
        DocuRequest,
    },
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, error, instrument};

use super::super::error::{map_ai_error, GenError};

use super::super::validators::validate_project_content::validate_project_content_request;
use super::request::ProjectContentRequest;

#[instrument(skip(model))]
#[actix_web::post("/project-content")]
pub async fn generate_project_content(
    model: web::Data<Arc<Client>>,
    payload: Json<ProjectContentRequest>,
) -> Result<HttpResponse, GenError> {
    // Validate payload
    validate_project_content_request(&payload)?;

    debug!(
        ">> Generating project content based on the provided project info: {:#?}",
        payload.project_info
    );

    // Generate project content
    let result =
        generate_project_content_with_validation(model.as_ref(), &payload.project_info).await?;

    debug!("\n>> Generated project content: \n\n{:#?}\n", result);

    Ok(HttpResponse::Ok().json(result))
}

/// Secure project content generation with error mapping
async fn generate_project_content_with_validation(
    model: &Client,
    project_info: &ProjectInfo,
) -> Result<ProjectContent, GenError> {
    match docuai::request::<ProjectContent>(model, DocuRequest::ProjectContent(project_info), 100)
        .await
    {
        Ok(project_content) => Ok(project_content),
        Err(error) => {
            error!("Project content generation failed: {:?}", error);

            match error {
                DocuError::ApiRequest(ge) => {
                    error!(
                        "AI request failed. Error: {}, Status: {:?}",
                        ge.message, ge.code
                    );
                    Err(map_ai_error(ge))
                }
                _ => {
                    error!("Unrecoverable error in project content generation");
                    Err(GenError::UnknownError)
                }
            }
        }
    }
}
