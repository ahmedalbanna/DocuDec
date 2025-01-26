use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docuai::{
    docu_requests::{
        schemas::{
            Abstract, AdditionalInstruction, ProjectContent, ProjectDetails, ProjectInfo,
            ProjectStructure,
        },
        DocuRequest,
    },
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, error, instrument};

use super::super::error::{map_ai_error, GenError};

use super::super::validators::validate_project_details::validate_project_details_request;
use super::request::ProjectDetailsRequest;

#[instrument(skip(model))]
#[actix_web::post("/project-details")]
pub async fn generate_project_details(
    model: web::Data<Arc<Client>>,
    payload: Json<ProjectDetailsRequest>,
) -> Result<HttpResponse, GenError> {
    // Validate payload
    validate_project_details_request(&payload)?;

    debug!(">> Generating project details");

    // Generate additional instruction
    let result = generate_project_details_(
        model.as_ref(),
        &payload.project_info,
        &payload.project_content,
        &payload.project_structure,
        &payload.additional_instruction,
        &payload.r#abstract,
    )
    .await?;

    debug!("\n>> Generated project details: \n\n{:#?}\n", result);

    Ok(HttpResponse::Ok().json(result))
}

/// Secure additional instruction generation with error mapping
async fn generate_project_details_(
    model: &Client,
    project_info: &ProjectInfo,
    project_content: &ProjectContent,
    project_structure: &ProjectStructure,
    additional_instruction: &AdditionalInstruction,
    r#abstract: &Abstract,
) -> Result<ProjectDetails, GenError> {
    match docuai::request(
        model,
        DocuRequest::ProjectDetails(
            project_info,
            project_content,
            project_structure,
            additional_instruction,
            r#abstract,
        ),
        100,
    )
    .await
    {
        Ok(project_abstract) => Ok(project_abstract),
        Err(error) => {
            error!("Project details generation failed: {:?}", error);

            match error {
                DocuError::ApiRequest(ge) => {
                    error!(
                        "AI request failed. Error: {}, Status: {:?}",
                        ge.message, ge.code
                    );
                    Err(map_ai_error(&ge))
                }
                DocuError::JsonConversion(error) => {
                    error!("JsonConversion error: {}", error);
                    Err(GenError::SyntaxError(error.to_string()))
                }
                error => {
                    error!(
                        "Unrecoverable error in project details generation: {}",
                        error
                    );
                    Err(GenError::UnknownError)
                }
            }
        }
    }
}
