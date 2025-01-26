use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docuai::{
    docu_requests::{
        schemas::{Abstract, AdditionalInstruction, ProjectContent, ProjectInfo, ProjectStructure},
        DocuRequest,
    },
    DocuError,
};
use docubase::error::DocuBaseError;
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, error, instrument};

use super::super::error::{map_ai_error, GenError};

use super::super::validators::validate_abstract::validate_abstract_request;
use super::request::AbstractRequest;

#[instrument(skip(model))]
#[actix_web::post("/abstract")]
pub async fn generate_abstraction(
    model: web::Data<Arc<Client>>,
    payload: Json<AbstractRequest>,
) -> Result<HttpResponse, GenError> {
    // Validate payload
    validate_abstract_request(&payload)?;

    debug!(">> Generating project abstraction");

    // Generate additional instruction
    let result = generate_abstraction_(
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
async fn generate_abstraction_(
    model: &Client,
    project_info: &ProjectInfo,
    project_content: &ProjectContent,
    project_structure: &ProjectStructure,
    additional_instruction: &AdditionalInstruction,
) -> Result<Abstract, GenError> {
    match docuai::request(
        model,
        DocuRequest::Abstract(
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

            if let DocuError::ApiRequest(ge) = error {
                error!(
                    "AI request failed. Error: {}, Status: {:?}",
                    ge.message, ge.code
                );
                Err(map_ai_error(&ge))
            } else {
                error!("Unrecoverable error in project abstraction generation");
                Err(GenError::UnknownError)
            }
        }
    }
}
