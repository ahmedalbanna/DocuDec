use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docuai::{
    docu_requests::{
        schemas::{
            Abstract, AdditionalInstruction, ProjectConclusion, ProjectContent, ProjectDetails,
            ProjectInfo, ProjectReferences, ProjectStructure,
        },
        DocuRequest,
    },
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, error, instrument};

use super::super::error::{map_ai_error, GenError};

use super::super::validators::validate_project_references::validate_project_references_request;
use super::request::ProjectReferencesRequest;

#[instrument(skip(model))]
#[actix_web::post("/project-references")]
pub async fn generate_project_references(
    model: web::Data<Arc<Client>>,
    payload: Json<ProjectReferencesRequest>,
) -> Result<HttpResponse, GenError> {
    // Validate payload
    validate_project_references_request(&payload)?;

    debug!(">> Generating project references");

    println!("{:#?}", payload);

    let result = generate_project_references_(
        model.as_ref(),
        &payload.project_info,
        &payload.project_content,
        &payload.project_structure,
        &payload.additional_instruction,
        &payload.r#abstract,
        &payload.project_details,
        &payload.project_conclusion,
    )
    .await?;

    debug!("\n>> Generated project references: \n\n{:#?}\n", result);

    Ok(HttpResponse::Ok().json(result))
}

// FIXME maybe create a struct that holds the data
#[allow(clippy::too_many_arguments)]
/// Secure additional instruction generation with error mapping
async fn generate_project_references_(
    model: &Client,
    project_info: &ProjectInfo,
    project_content: &ProjectContent,
    project_structure: &ProjectStructure,
    additional_instruction: &AdditionalInstruction,
    r#abstract: &Abstract,
    project_details: &ProjectDetails,
    project_conclusion: &ProjectConclusion,
) -> Result<ProjectReferences, GenError> {
    match docuai::request(
        model,
        DocuRequest::ProjectReferences(
            project_info,
            project_content,
            project_structure,
            additional_instruction,
            r#abstract,
            project_details,
            project_conclusion,
        ),
        100,
    )
    .await
    {
        Ok(project_references) => Ok(project_references),
        Err(error) => {
            error!("Project references generation failed: {:?}", error);

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
