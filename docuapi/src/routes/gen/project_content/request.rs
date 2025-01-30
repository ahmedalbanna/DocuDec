use docuai::docu_requests::schemas::{HasProjectInfo, ProjectInfo};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectContentRequest {
    pub project_id: Uuid,
    pub project_info: Option<ProjectInfo>,
}

impl HasProjectInfo for ProjectContentRequest {
    fn get_project_info(&self) -> Option<&ProjectInfo> {
        self.project_info.as_ref()
    }
}
