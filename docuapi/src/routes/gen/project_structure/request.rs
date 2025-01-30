use docuai::docu_requests::schemas::{
    project_content::ProjectContent, project_info::ProjectInfo, HasProjectContent, HasProjectInfo,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectStructureRequest {
    pub project_id: Uuid,
    pub project_info: Option<ProjectInfo>,
    pub project_content: Option<ProjectContent>,
}

impl HasProjectInfo for ProjectStructureRequest {
    fn get_project_info(&self) -> Option<&ProjectInfo> {
        self.project_info.as_ref()
    }
}

impl HasProjectContent for ProjectStructureRequest {
    fn get_project_content(&self) -> Option<&ProjectContent> {
        self.project_content.as_ref()
    }
}
