use docuai::docu_requests::schemas::{ProjectContent, ProjectInfo, ProjectStructure};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::gen::ProvidesReport;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AdditionalInstructionRequest {
    pub project_id: Uuid,
    pub update: Option<bool>,
    pub project_info: Option<ProjectInfo>,
    pub project_content: Option<ProjectContent>,
    pub project_structure: Option<ProjectStructure>,
}

impl ProvidesReport<ProjectInfo> for AdditionalInstructionRequest {
    fn get_report(&self) -> Option<&ProjectInfo> {
        self.project_info.as_ref()
    }
}

impl ProvidesReport<ProjectContent> for AdditionalInstructionRequest {
    fn get_report(&self) -> Option<&ProjectContent> {
        self.project_content.as_ref()
    }
}

impl ProvidesReport<ProjectStructure> for AdditionalInstructionRequest {
    fn get_report(&self) -> Option<&ProjectStructure> {
        self.project_structure.as_ref()
    }
}
