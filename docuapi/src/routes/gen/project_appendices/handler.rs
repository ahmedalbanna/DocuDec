use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docuai::{
    docu_requests::{
        schemas::{
            Abstract, AdditionalInstruction, ProjectAppendices, ProjectConclusion, ProjectContent,
            ProjectDetails, ProjectInfo, ProjectReferences, ProjectStructure,
        },
        DocuRequest,
    },
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, error, instrument};

use super::super::error::{map_ai_error, GenError};

use super::super::validators::validate_project_appendices::validate_project_appendices_request;
use super::request::ProjectAppendicesRequest;

#[instrument(skip(model))]
#[actix_web::post("/project-appendices")]
pub async fn generate_project_appendices(
    model: web::Data<Arc<Client>>,
    payload: Json<ProjectAppendicesRequest>,
) -> Result<HttpResponse, GenError> {
    // Validate payload
    validate_project_appendices_request(&payload)?;

    debug!(">> Generating project appendices");

    let result = generate_project_appendices_(
        model.as_ref(),
        &payload.project_info,
        &payload.project_content,
        &payload.project_structure,
        &payload.additional_instruction,
        &payload.r#abstract,
        &payload.project_details,
        &payload.project_conclusion,
        &payload.project_references,
    )
    .await?;

    debug!("\n>> Generated project appendices: \n\n{:#?}\n", result);

    Ok(HttpResponse::Ok().json(result))
}

// FIXME maybe create a struct that holds the data
#[allow(clippy::too_many_arguments)]
/// Secure additional instruction generation with error mapping
async fn generate_project_appendices_(
    model: &Client,
    project_info: &ProjectInfo,
    project_content: &ProjectContent,
    project_structure: &ProjectStructure,
    additional_instruction: &AdditionalInstruction,
    r#abstract: &Abstract,
    project_details: &ProjectDetails,
    project_conclusion: &ProjectConclusion,
    project_references: &ProjectReferences,
) -> Result<ProjectAppendices, GenError> {
    match docuai::request(
        model,
        DocuRequest::ProjectAppendices(
            project_info,
            project_content,
            project_structure,
            additional_instruction,
            r#abstract,
            project_details,
            project_conclusion,
            project_references,
        ),
        100,
    )
    .await
    {
        Ok(project_appendices) => Ok(project_appendices),
        Err(error) => {
            error!("Project appendices generation failed: {:?}", error);

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
