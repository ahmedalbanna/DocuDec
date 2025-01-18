use docuai::docu_requests::schemas::{
    Abstract, AdditionalInstruction, ProjectConclusion, ProjectContent, ProjectDetails,
    ProjectInfo, ProjectStructure,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectReferencesRequest {
    pub project_info: ProjectInfo,
    pub project_content: ProjectContent,
    pub project_structure: ProjectStructure,
    pub additional_instruction: AdditionalInstruction,
    pub r#abstract: Abstract,
    pub project_details: ProjectDetails,
    pub project_conclusion: ProjectConclusion,
}
