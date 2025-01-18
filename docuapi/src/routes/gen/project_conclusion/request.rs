use docuai::docu_requests::schemas::{
    Abstract, AdditionalInstruction, ProjectContent, ProjectDetails, ProjectInfo, ProjectStructure,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectConclusionRequest {
    pub project_info: ProjectInfo,
    pub project_content: ProjectContent,
    pub project_structure: ProjectStructure,
    pub additional_instruction: AdditionalInstruction,
    pub r#abstract: Abstract,
    pub project_details: ProjectDetails,
}
