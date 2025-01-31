use docuai::docu_requests::schemas::{
    AdditionalInstruction, ProjectAbstract, ProjectConclusion, ProjectContent, ProjectInfo,
    ProjectReferences, ProjectStructure,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::gen::ProvidesReport;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectAppendicesRequest {
    pub project_id: Uuid,
    pub project_info: Option<ProjectInfo>,
    pub project_content: Option<ProjectContent>,
    pub project_structure: Option<ProjectStructure>,
    pub additional_instruction: Option<AdditionalInstruction>,
    pub project_abstract: Option<ProjectAbstract>,
    pub project_conclusion: Option<ProjectConclusion>,
    pub project_references: Option<ProjectReferences>,
}

impl ProvidesReport<ProjectInfo> for ProjectAppendicesRequest {
    fn get_report(&self) -> Option<&ProjectInfo> {
        self.project_info.as_ref()
    }
}

impl ProvidesReport<ProjectContent> for ProjectAppendicesRequest {
    fn get_report(&self) -> Option<&ProjectContent> {
        self.project_content.as_ref()
    }
}

impl ProvidesReport<ProjectStructure> for ProjectAppendicesRequest {
    fn get_report(&self) -> Option<&ProjectStructure> {
        self.project_structure.as_ref()
    }
}

impl ProvidesReport<AdditionalInstruction> for ProjectAppendicesRequest {
    fn get_report(&self) -> Option<&AdditionalInstruction> {
        self.additional_instruction.as_ref()
    }
}

impl ProvidesReport<ProjectAbstract> for ProjectAppendicesRequest {
    fn get_report(&self) -> Option<&ProjectAbstract> {
        self.project_abstract.as_ref()
    }
}

impl ProvidesReport<ProjectConclusion> for ProjectAppendicesRequest {
    fn get_report(&self) -> Option<&ProjectConclusion> {
        self.project_conclusion.as_ref()
    }
}

impl ProvidesReport<ProjectReferences> for ProjectAppendicesRequest {
    fn get_report(&self) -> Option<&ProjectReferences> {
        self.project_references.as_ref()
    }
}
