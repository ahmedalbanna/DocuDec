use actix_web::web::Json;
use tracing::error;

use super::super::additional_instruction::request::AdditionalInstructionRequest;
use super::super::error::GenError;

pub fn validate_additional_instructions_request(
    payload: &Json<AdditionalInstructionRequest>,
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

    Ok(())
}
