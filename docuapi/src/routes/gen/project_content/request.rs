use docuai::docu_requests::schemas::ProjectInfo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::gen::ProvidesReport;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectContentRequest {
    pub project_id: Uuid,
    pub update: Option<bool>,
    pub update_if_exist: Option<bool>,
    pub project_info: Option<ProjectInfo>,
}

impl ProvidesReport<ProjectInfo> for ProjectContentRequest {
    fn get_report(&self) -> Option<&ProjectInfo> {
        self.project_info.as_ref()
    }
}
