use docuai::docu_requests::schemas::{
    Abstract, AdditionalInstruction, ProjectContent, ProjectInfo, ProjectStructure,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectDetailsRequest {
    pub project_info: ProjectInfo,
    pub project_content: ProjectContent,
    pub project_structure: ProjectStructure,
    pub additional_instruction: AdditionalInstruction,
    pub r#abstract: Abstract,
}
