use docuai::docu_requests::schemas::{project_content::ProjectContent, project_info::ProjectInfo};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::gen::ProvidesReport;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectStructureRequest {
    pub project_id: Uuid,
    pub update: Option<bool>,
    pub project_info: Option<ProjectInfo>,
    pub project_content: Option<ProjectContent>,
}

impl ProvidesReport<ProjectInfo> for ProjectStructureRequest {
    fn get_report(&self) -> Option<&ProjectInfo> {
        self.project_info.as_ref()
    }
}

impl ProvidesReport<ProjectContent> for ProjectStructureRequest {
    fn get_report(&self) -> Option<&ProjectContent> {
        self.project_content.as_ref()
    }
}
