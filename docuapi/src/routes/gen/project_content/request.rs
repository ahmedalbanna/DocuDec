use docuai::docu_requests::schemas::ProjectInfo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectContentRequest {
    pub project_id: Uuid,
    pub project_info: ProjectInfo,
}
