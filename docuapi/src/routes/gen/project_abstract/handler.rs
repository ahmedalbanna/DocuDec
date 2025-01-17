use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docuai::{
    docu_requests::{
        schemas::{
            additional_instruction::AdditionalInstruction, project_abstract::ProjectAbstract,
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

use super::super::validators::validate_project_abstract::validate_project_abstract_request;
use super::request::ProjectAbstractRequest;

#[instrument(skip(model))]
#[actix_web::post("/project-abstract")]
pub async fn generate_project_abstraction(
    model: web::Data<Arc<Client>>,
    payload: Json<ProjectAbstractRequest>,
) -> Result<HttpResponse, GenError> {
    // Validate payload
    validate_project_abstract_request(&payload)?;

    debug!(">> Generating project abstraction");

    // Generate additional instruction
    let result = generate_project_abstraction_(
        model.as_ref(),
        &payload.project_info,
        &payload.project_content,
        &payload.project_structure,
        &payload.additional_instruction,
    )
    .await?;

    debug!("\n>> Generated project abstraction: \n\n{:#?}\n", result);

    Ok(HttpResponse::Ok().json(result))
}

/// Secure additional instruction generation with error mapping
async fn generate_project_abstraction_(
    model: &Client,
    project_info: &ProjectInfo,
    project_content: &ProjectContent,
    project_structure: &ProjectStructure,
    additional_instruction: &AdditionalInstruction,
) -> Result<ProjectAbstract, GenError> {
    match docuai::request::<ProjectAbstract>(
        model,
        DocuRequest::ProjectAbstract(
            project_info,
            project_content,
            project_structure,
            additional_instruction,
        ),
        100,
    )
    .await
    {
        Ok(project_abstract) => Ok(project_abstract),
        Err(error) => {
            error!("Project abstraction generation failed: {:?}", error);

            match error {
                DocuError::ApiRequest(ge) => {
                    error!(
                        "AI request failed. Error: {}, Status: {:?}",
                        ge.message, ge.code
                    );
                    Err(map_ai_error(ge))
                }
                _ => {
                    error!("Unrecoverable error in project abstraction generation");
                    Err(GenError::UnknownError)
                }
            }
        }
    }
}
