use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docuai::{
    docu_requests::{
        schemas::{
            Abstract, AdditionalInstruction, ProjectConclusion, ProjectContent, ProjectDetails,
            ProjectInfo, ProjectStructure,
        },
        DocuRequest,
    },
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, error, instrument};

use super::super::error::{map_ai_error, GenError};

use super::super::validators::validate_project_conclusion::validate_project_conclusion_request;
use super::request::ProjectConclusionRequest;

#[instrument(skip(model))]
#[actix_web::post("/project-conclusion")]
pub async fn generate_project_conclusion(
    model: web::Data<Arc<Client>>,
    payload: Json<ProjectConclusionRequest>,
) -> Result<HttpResponse, GenError> {
    // Validate payload
    validate_project_conclusion_request(&payload)?;

    debug!(">> Generating project conclusion");

    // Generate additional instruction
    let result = generate_project_conclusion_(
        model.as_ref(),
        &payload.project_info,
        &payload.project_content,
        &payload.project_structure,
        &payload.additional_instruction,
        &payload.r#abstract,
        &payload.project_details,
    )
    .await?;

    debug!("\n>> Generated project conclusion: \n\n{:#?}\n", result);

    Ok(HttpResponse::Ok().json(result))
}

/// Secure additional instruction generation with error mapping
async fn generate_project_conclusion_(
    model: &Client,
    project_info: &ProjectInfo,
    project_content: &ProjectContent,
    project_structure: &ProjectStructure,
    additional_instruction: &AdditionalInstruction,
    r#abstract: &Abstract,
    project_details: &ProjectDetails,
) -> Result<ProjectConclusion, GenError> {
    match docuai::request(
        model,
        DocuRequest::ProjectConclusion(
            project_info,
            project_content,
            project_structure,
            additional_instruction,
            r#abstract,
            project_details,
        ),
        100,
    )
    .await
    {
        Ok(project_conclusion) => Ok(project_conclusion),
        Err(error) => {
            error!("Project conclusion generation failed: {:?}", error);

            match error {
                DocuError::ApiRequest(ge) => {
                    error!(
                        "AI request failed. Error: {}, Status: {:?}",
                        ge.message, ge.code
                    );
                    Err(map_ai_error(ge))
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
