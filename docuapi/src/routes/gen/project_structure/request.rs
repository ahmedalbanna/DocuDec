use docuai::docu_requests::schemas::{project_content::ProjectContent, project_info::ProjectInfo};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectStructureRequest {
    pub project_id: Uuid,
    pub project_info: ProjectInfo,
    pub project_content: ProjectContent,
}
