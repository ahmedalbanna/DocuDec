use docuai::docu_requests::schemas::ProjectInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectContentRequest {
    pub project_info: ProjectInfo,
}
