use actix_web::web::Json;
use tracing::error;

use super::super::error::GenError;
use super::super::project_abstract::request::ProjectAbstractRequest;

pub fn validate_project_abstract_request(
    payload: &Json<ProjectAbstractRequest>,
) -> Result<(), GenError> {
    // Validate project_info
    if payload.project_info.project_title.is_empty() {
        error!("Project info title cannot be empty");
        return Err(GenError::InvalidTitle(
            "Project info title cannot be empty".into(),
        ));
    }

    // Validate project_content
    if payload
        .project_content
        .project_content
        .future_work
        .description
        .is_empty()
    {
        error!("Project content future work cannot be empty");
        return Err(GenError::MissingContent(
            "Project content sections are required".to_string(),
        ));
    }

    // Validate project_structure
    if payload
        .project_structure
        .structure_and_format
        .target_length
        .is_empty()
    {
        error!("Project structure target length cannot be empty");
        return Err(GenError::MissingContent(
            "Project structure is required".to_string(),
        ));
    }

    // Validate project_structure
    if payload
        .additional_instruction
        .additional_instructions
        .tone_and_style
        .is_empty()
    {
        error!("Additional instruction tone and style cannot be empty");
        return Err(GenError::MissingContent(
            "Additional instruction is required".to_string(),
        ));
    }

    Ok(())
}
