use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docuai::{
    docu_requests::{
        responses::{
            project_content::ProjectContent, project_info::ProjectInfo,
            project_structure::ProjectStructure,
        },
        DocuRequest,
    },
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, error, instrument};

use super::super::error::{map_ai_error, GenError};

use super::super::validators::validate_project_structure::validate_project_structure_request;
use super::request::ProjectStructureRequest;

#[instrument(skip(model))]
#[actix_web::post("/project-structure")]
pub async fn generate_project_structure(
    model: web::Data<Arc<Client>>,
    payload: Json<ProjectStructureRequest>,
) -> Result<HttpResponse, GenError> {
    // Validate payload
    validate_project_structure_request(&payload)?;

    debug!(
        ">> Generating project structure | Project Info: {:#?} | Project Content: {:#?}",
        payload.project_info, payload.project_content
    );

    // Generate project structure
    let result = generate_project_structure_with_validation(
        model.as_ref(),
        &payload.project_info,
        &payload.project_content,
    )
    .await?;

    debug!("\n>> Generated project structure: \n\n{:#?}\n", result);

    Ok(HttpResponse::Ok().json(result))
}

/// Secure project structure generation with error mapping
async fn generate_project_structure_with_validation(
    model: &Client,
    project_info: &ProjectInfo,
    project_content: &ProjectContent,
) -> Result<ProjectStructure, GenError> {
    match docuai::request::<ProjectStructure>(
        model,
        DocuRequest::ProjectStructure(project_info, project_content),
        100,
    )
    .await
    {
        Ok(project_structure) => Ok(project_structure),
        Err(error) => {
            error!("Project structure generation failed: {:?}", error);

            match error {
                DocuError::ApiRequest(ge) => {
                    error!(
                        "AI request failed. Error: {}, Status: {:?}",
                        ge.message, ge.code
                    );
                    Err(map_ai_error(ge))
                }
                _ => {
                    error!("Unrecoverable error in project structure generation");
                    Err(GenError::UnknownError)
                }
            }
        }
    }
}
