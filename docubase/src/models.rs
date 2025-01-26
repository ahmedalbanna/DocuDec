mod project;
mod report;
mod user;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

pub use project::Project;
pub use report::{Report, ReportMetadata, ReportType};
pub use user::User;

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateResponse {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct DeleteResponse {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Claims {
    /// User ID
    pub sub: String,

    /// Expiration timestamp
    pub exp: u64,
}
