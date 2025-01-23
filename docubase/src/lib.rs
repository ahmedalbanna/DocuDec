//pub mod auth;
mod constants;
mod error;
pub mod models;
pub mod utils;

use constants::DATABASE_URL;
use error::DocuBaseError;
use models::{CreateResponse, DeleteResponse, Project, Report, ReportMetadata, ReportType};
use sqlx::{postgres::PgPool, Pool, Postgres};
use utils::{generate_jwt, hash_password, validate_jwt, verify_password};
use uuid::Uuid;

pub struct DocuBase {
    pool: Pool<Postgres>,
}

enum LookupField<'a> {
    Username(&'a str),
    Id(&'a Uuid),
}

impl DocuBase {
    pub async fn new() -> Result<Self, DocuBaseError> {
        dotenv::dotenv().ok();
        let db_url = &*DATABASE_URL;

        let pool = PgPool::connect(db_url).await?;

        let password = "abdullah-albanna".to_string();

        let hashed_password = hash_password(&password).unwrap();

        println!("password: {}", password);
        println!("hashed password: {}", hashed_password);

        println!(
            "verification result: {}",
            verify_password(&password, &hashed_password).unwrap()
        );

        let jwt = generate_jwt("13").unwrap();
        println!("jwt for the 13 as the user_id: {}", jwt);

        println!(
            "extracted user_id from the jwt: {}",
            validate_jwt(&jwt).unwrap()
        );

        Ok(Self { pool })
    }
}

// user
impl DocuBase {
    async fn user_exists(&self, field: LookupField<'_>) -> Result<bool, DocuBaseError> {
        let user = match field {
            LookupField::Username(username) => {
                sqlx::query!(
                    r#"SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)"#,
                    username
                )
                .fetch_one(&self.pool)
                .await?
                .exists
            }
            LookupField::Id(id) => {
                sqlx::query!(r#"SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)"#, id)
                    .fetch_one(&self.pool)
                    .await?
                    .exists
            }
        };

        Ok(user.unwrap_or(false))
    }

    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<CreateResponse, DocuBaseError> {
        if self.user_exists(LookupField::Username(&username)).await? {
            return Err(DocuBaseError::UserExists);
        }

        let hashed_password = utils::hash_password(&password)?;

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
        username: String,
        password: String,
    ) -> Result<DeleteResponse, DocuBaseError> {
        if !self.user_exists(LookupField::Username(&username)).await? {
            return Err(DocuBaseError::UserDoesNotExists);
        }

        let hashed_password = utils::hash_password(&password)?;

        let user = sqlx::query!(
            r#"
            DELETE FROM users WHERE username = $1 AND password = $2
            RETURNING id
            "#,
            username,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DeleteResponse { id: user.id })
    }
}

// project
impl DocuBase {
    pub async fn project_exists(&self, id: &Uuid) -> Result<bool, DocuBaseError> {
        let project = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM projects WHERE id = $1)
           "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(project.exists.unwrap_or(false))
    }

    pub async fn get_user_projects(&self, user_id: Uuid) -> Result<Vec<Project>, DocuBaseError> {
        if !self.user_exists(LookupField::Id(&user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExists);
        }

        let projects = sqlx::query!(
            r#"
                SELECT id, title from projects WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(projects
            .into_iter()
            .map(|record| Project {
                user_id,
                project_id: record.id,
                title: record.title,
            })
            .collect::<Vec<_>>())
    }

    pub async fn get_project(&self, project_id: Uuid) -> Result<Project, DocuBaseError> {
        if !self.project_exists(&project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExists);
        }

        let project = sqlx::query!(
            r#"
                SELECT id, title, user_id from projects WHERE id = $1
            "#,
            project_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Project {
            project_id,
            user_id: project.user_id,
            title: project.title,
        })
    }

    pub async fn get_project_state(&self, project_id: Uuid) -> Result<ReportType, DocuBaseError> {
        if !self.project_exists(&project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExists);
        }

        let project = sqlx::query!(
            r#"
                SELECT state from projects where id = $1
            "#,
            project_id
        )
        .fetch_one(&self.pool)
        .await?;

        let state = project.state.ok_or(DocuBaseError::ProjectStateNotFound)?;

        ReportType::try_from(state)
    }

    pub async fn create_project(
        &self,
        user_id: Uuid,
        title: String,
    ) -> Result<CreateResponse, DocuBaseError> {
        if !self.user_exists(LookupField::Id(&user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExists);
        }

        let project = sqlx::query!(
            r#"
            INSERT INTO projects (user_id, title)
            VALUES ($1, $2)
            RETURNING id, created_at
            "#,
            user_id,
            title,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CreateResponse {
            id: project.id,
            created_at: project.created_at,
        })
    }

    pub async fn delete_project(&self, project_id: Uuid) -> Result<DeleteResponse, DocuBaseError> {
        if !self.project_exists(&project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExists);
        }

        let project = sqlx::query!(
            r#"
            DELETE FROM projects WHERE id = $1
            RETURNING id
            "#,
            project_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DeleteResponse { id: project.id })
    }
}

// report
impl DocuBase {
    pub async fn report_exists(&self, id: &Uuid) -> Result<bool, DocuBaseError> {
        let report = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM reports WHERE id = $1)
           "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(report.exists.unwrap_or(false))
    }

    pub async fn create_report<R: ReportMetadata>(
        &self,
        project_id: Uuid,
        report: R,
    ) -> Result<CreateResponse, DocuBaseError> {
        if !self.project_exists(&project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExists);
        }

        let report = sqlx::query!(
            r#"
            INSERT INTO reports (project_id, type, content)
            VALUES ($1, $2, $3)
            RETURNING id, created_at
            "#,
            project_id,
            report.report_type().to_string(),
            report.content(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CreateResponse {
            id: report.id,
            created_at: report.created_at,
        })
    }

    pub async fn delete_report(&self, report_id: Uuid) -> Result<DeleteResponse, DocuBaseError> {
        if !self.report_exists(&report_id).await? {
            return Err(DocuBaseError::ReportDoesNotExists);
        }

        let report = sqlx::query!(
            r#"
            DELETE FROM reports WHERE id = $1 
            RETURNING id
            "#,
            report_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DeleteResponse { id: report.id })
    }

    pub async fn get_report(&self, report_id: Uuid) -> Result<Report, DocuBaseError> {
        if !self.report_exists(&report_id).await? {
            return Err(DocuBaseError::ReportDoesNotExists);
        }

        let report = sqlx::query!(
            r#"
                SELECT id, type, content, project_id from reports WHERE id = $1
            "#,
            report_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Report {
            project_id: report.project_id,
            report_id,
            r#type: report.r#type.try_into().unwrap(),
            content: report.content,
        })
    }

    pub async fn get_project_reports(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<Report>, DocuBaseError> {
        if !self.project_exists(&project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExists);
        }

        let reports = sqlx::query!(
            r#"
                SELECT id, type, content from reports WHERE project_id = $1
            "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        // TODO: get rid of the clone, and do better at error handling
        Ok(reports
            .into_iter()
            .map(|record| Report {
                project_id,
                report_id: record.id,
                r#type: record
                    .r#type
                    .clone()
                    .try_into()
                    .unwrap_or_else(|_| panic!("invalid report type, got: {}", record.r#type)),
                content: record.content,
            })
            .collect::<Vec<_>>())
    }
}
