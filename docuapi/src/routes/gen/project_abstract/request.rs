use docuai::docu_requests::schemas::{
    AdditionalInstruction, ProjectContent, ProjectInfo, ProjectStructure,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::gen::ProvidesReport;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectAbstractRequest {
    pub project_id: Uuid,
    pub update: Option<bool>,
    pub project_info: Option<ProjectInfo>,
    pub project_content: Option<ProjectContent>,
    pub project_structure: Option<ProjectStructure>,
    pub additional_instruction: Option<AdditionalInstruction>,
}

impl ProvidesReport<ProjectInfo> for ProjectAbstractRequest {
    fn get_report(&self) -> Option<&ProjectInfo> {
        self.project_info.as_ref()
    }
}

impl ProvidesReport<ProjectContent> for ProjectAbstractRequest {
    fn get_report(&self) -> Option<&ProjectContent> {
        self.project_content.as_ref()
    }
}

impl ProvidesReport<ProjectStructure> for ProjectAbstractRequest {
    fn get_report(&self) -> Option<&ProjectStructure> {
        self.project_structure.as_ref()
    }
}

impl ProvidesReport<AdditionalInstruction> for ProjectAbstractRequest {
    fn get_report(&self) -> Option<&AdditionalInstruction> {
        self.additional_instruction.as_ref()
    }
}
