use docuai::docu_requests::schemas::{
    AdditionalInstruction, ProjectAbstract, ProjectContent, ProjectInfo, ProjectStructure,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectConclusionRequest {
    pub project_id: Uuid,
    pub project_info: ProjectInfo,
    pub project_content: ProjectContent,
    pub project_structure: ProjectStructure,
    pub additional_instruction: AdditionalInstruction,
    pub project_abstract: ProjectAbstract,
}
