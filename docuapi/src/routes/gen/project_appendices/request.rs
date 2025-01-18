use docuai::docu_requests::schemas::{
    Abstract, AdditionalInstruction, ProjectConclusion, ProjectContent, ProjectDetails,
    ProjectInfo, ProjectReferences, ProjectStructure,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectAppendicesRequest {
    pub project_info: ProjectInfo,
    pub project_content: ProjectContent,
    pub project_structure: ProjectStructure,
    pub additional_instruction: AdditionalInstruction,
    pub r#abstract: Abstract,
    pub project_details: ProjectDetails,
    pub project_conclusion: ProjectConclusion,
    pub project_references: ProjectReferences,
}
