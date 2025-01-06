use docuai::docu_requests::responses::{
    additional_instruction::AdditionalInstruction, project_content::ProjectContent,
    project_info::ProjectInfo, project_structure::ProjectStructure,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectAbstractRequest {
    pub project_info: ProjectInfo,
    pub project_content: ProjectContent,
    pub project_structure: ProjectStructure,
    pub additional_instruction: AdditionalInstruction,
}
