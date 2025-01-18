use docuai::docu_requests::schemas::{
    AdditionalInstruction, ProjectContent, ProjectInfo, ProjectStructure,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AbstractRequest {
    pub project_info: ProjectInfo,
    pub project_content: ProjectContent,
    pub project_structure: ProjectStructure,
    pub additional_instruction: AdditionalInstruction,
}
