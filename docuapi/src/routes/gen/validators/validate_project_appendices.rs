use actix_web::web::Json;
use tracing::error;

use crate::routes::gen::project_appendices::request::ProjectAppendicesRequest;

use super::super::error::GenError;

pub fn validate_project_appendices_request(
    payload: &Json<ProjectAppendicesRequest>,
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
