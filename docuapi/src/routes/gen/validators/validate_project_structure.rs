use actix_web::web::Json;
use tracing::error;

use super::super::error::GenError;
use super::super::project_structure::request::ProjectStructureRequest;

pub fn validate_project_structure_request(
    payload: &Json<ProjectStructureRequest>,
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

    Ok(())
}
