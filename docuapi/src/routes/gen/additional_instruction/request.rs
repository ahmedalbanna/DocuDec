use docuai::docu_requests::schemas::{ProjectContent, ProjectInfo, ProjectStructure};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AdditionalInstructionRequest {
    pub project_id: Uuid,
    pub project_info: ProjectInfo,
    pub project_content: ProjectContent,
    pub project_structure: ProjectStructure,
}
