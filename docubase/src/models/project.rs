use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub title: String,
}
