pub mod auth;
mod error;
pub mod models;
pub mod utils;

use error::DocuBaseError;
use models::{CreateResponse, DeleteResponse};
use serde_json::Value;
use sqlx::{postgres::PgPool, Error as SqlxError, Pool, Postgres};
use uuid::Uuid;

pub struct DocuBase {
    pool: Pool<Postgres>,
}

impl DocuBase {
    pub async fn new() -> Result<Self, SqlxError> {
        let db_url = std::env::var("DATABASE_URL")
            .expect("Failed to get the DATABASE_URL env, make sure to set it");

        let pool = PgPool::connect(&db_url).await?;

        Ok(Self { pool })
    }

    pub async fn user_exists_from_username(&self, username: &String) -> Result<bool, SqlxError> {
        let user = sqlx::query!(
            r#"
            SELECT * FROM users WHERE username = $1
           "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.is_some())
    }

    pub async fn user_exists_from_email(&self, email: &String) -> Result<bool, SqlxError> {
        let user = sqlx::query!(
            r#"
            SELECT * FROM users WHERE email = $1
           "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.is_some())
    }

    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<CreateResponse, DocuBaseError> {
        let hashed_password = utils::hash_password(&password).unwrap();

        if self.user_exists_from_username(&username).await? {
            return Err(DocuBaseError::UserExists);
        }

        let user = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, created_at
            "#,
            username,
            email,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CreateResponse {
            id: user.id,
            created_at: user.created_at,
        })
    }

    pub async fn delete_user(
        &self,
        email: String,
        password: String,
    ) -> Result<DeleteResponse, DocuBaseError> {
        let hashed_password = utils::hash_password(&password).unwrap();

        if !self.user_exists_from_email(&email).await? {
            return Err(DocuBaseError::UserDoesNotExists);
        }

        let user = sqlx::query!(
            r#"
            DELETE FROM users WHERE email = $1 AND password = $2
            RETURNING id
            "#,
            email,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DeleteResponse { id: user.id })
    }

    pub async fn create_project(
        &self,
        user_id: Uuid,
        title: String,
        information: Value,
    ) -> Result<CreateResponse, SqlxError> {
        let project = sqlx::query!(
            r#"
            INSERT INTO projects (user_id, title, information)
            VALUES ($1, $2, $3)
            RETURNING id, created_at
            "#,
            user_id,
            title,
            information
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CreateResponse {
            id: project.id,
            created_at: project.created_at,
        })
    }

    pub async fn delete_project(
        &self,
        user_id: Uuid,
        project_id: Uuid,
    ) -> Result<DeleteResponse, SqlxError> {
        let project = sqlx::query!(
            r#"
            DELETE FROM projects WHERE id = $1 AND user_id = $2
            RETURNING id
            "#,
            project_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DeleteResponse { id: project.id })
    }

    pub async fn create_report(
        &self,
        project_id: Uuid,
        content: Value,
    ) -> Result<CreateResponse, SqlxError> {
        let report = sqlx::query!(
            r#"
            INSERT INTO reports (project_id, content)
            VALUES ($1, $2)
            RETURNING id, created_at
            "#,
            project_id,
            content,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CreateResponse {
            id: report.id,
            created_at: report.created_at,
        })
    }

    pub async fn delete_report(
        &self,
        project_id: Uuid,
        report_id: Uuid,
    ) -> Result<DeleteResponse, SqlxError> {
        let report = sqlx::query!(
            r#"
            DELETE FROM reports WHERE id = $1 AND project_id = $2
            RETURNING id
            "#,
            report_id,
            project_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DeleteResponse { id: report.id })
    }
}
