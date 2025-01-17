use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

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
pub struct User {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration timestamp
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Project {
    id: Uuid,
    user_id: Uuid,
    title: String,
    information: Value,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Report {
    id: Uuid,
    project_id: Uuid,
    content: Value,
}
