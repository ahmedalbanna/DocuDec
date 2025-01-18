use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docuai::{
    docu_requests::{
        schemas::{AdditionalInstruction, ProjectContent, ProjectInfo, ProjectStructure},
        DocuRequest,
    },
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, error, instrument};

use super::super::error::{map_ai_error, GenError};

use super::super::validators::validate_additional_instruction::validate_additional_instructions_request;
use super::request::AdditionalInstructionRequest;

#[instrument(skip(model))]
#[actix_web::post("/additional-instruction")]
pub async fn generate_additional_instruction(
    model: web::Data<Arc<Client>>,
    payload: Json<AdditionalInstructionRequest>,
) -> Result<HttpResponse, GenError> {
    // Validate payload
    validate_additional_instructions_request(&payload)?;

    debug!(">> Generating additional instruction");

    // Generate additional instruction
    let result = generate_additional_instruction_(
        model.as_ref(),
        &payload.project_info,
        &payload.project_content,
        &payload.project_structure,
    )
    .await?;

    debug!("\n>> Generated additional instruction: \n\n{:#?}\n", result);

    Ok(HttpResponse::Ok().json(result))
}

/// Secure additional instruction generation with error mapping
async fn generate_additional_instruction_(
    model: &Client,
    project_info: &ProjectInfo,
    project_content: &ProjectContent,
    project_structure: &ProjectStructure,
) -> Result<AdditionalInstruction, GenError> {
    match docuai::request::<AdditionalInstruction>(
        model,
        DocuRequest::AdditionalInstruction(project_info, project_content, project_structure),
        100,
    )
    .await
    {
        Ok(additional_instruction) => Ok(additional_instruction),
        Err(error) => {
            error!("Additional instruction generation failed: {:?}", error);

            match error {
                DocuError::ApiRequest(ge) => {
                    error!(
                        "AI request failed. Error: {}, Status: {:?}",
                        ge.message, ge.code
                    );
                    Err(map_ai_error(ge))
                }
                _ => {
                    error!("Unrecoverable error in additional instruction generation");
                    Err(GenError::UnknownError)
                }
            }
        }
    }
}
