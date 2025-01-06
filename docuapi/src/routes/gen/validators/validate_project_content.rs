use actix_web::web::Json;
use tracing::error;

use super::super::project_content::request::ProjectContentRequest;

use super::super::error::GenError;

pub fn validate_project_content_request(
    payload: &Json<ProjectContentRequest>,
) -> Result<(), GenError> {
    if payload.project_info.project_title.is_empty() {
        error!("Project info title cannot be empty");
        return Err(GenError::InvalidTitle(
            "Project info title cannot be empty".into(),
        ));
    }

    Ok(())
}
