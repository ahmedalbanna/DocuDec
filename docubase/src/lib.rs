//pub mod auth;
mod constants;
pub mod error;
pub mod models;
pub mod utils;

use std::time::Duration;

use constants::DATABASE_URL;
use error::DocuBaseError;
use models::{CreateResponse, DeleteResponse, Project, Report, ReportMetadata, ReportType};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

pub struct DocuBase {
    pool: Pool<Postgres>,
}

enum LookupField<'a> {
    Username(&'a str),
    Email(&'a str),
    Id(&'a Uuid),
}

impl DocuBase {
    /// # Errors
    /// it will return
    /// ```rust
    /// DocuBaseError::SqlxError(...)
    /// ```
    ///  if the ``PgPool`` (Postgres database) failed
    ///
    ///  could be because of the url, or the database is not avaliable
    pub async fn new() -> Result<Self, DocuBaseError> {
        let db_url = &*DATABASE_URL;

        let pool = PgPoolOptions::new()
            .max_connections(50)
            .min_connections(10)
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(10))
            .max_lifetime(Duration::from_secs(1800))
            .test_before_acquire(true)
            .connect(db_url)
            .await?;

        Ok(Self { pool })
    }
}

// user
impl DocuBase {
    /// # Errors
    /// ```rust
    /// DocuBaseError::SqlxError(sqlx::Error)
    /// ```
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
            LookupField::Email(email) => {
                sqlx::query!(
                    r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"#,
                    email
                )
                .fetch_one(&self.pool)
                .await?
                .exists
            }
        };

        Ok(user.unwrap_or(false))
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::SqlxError
    /// ```
    pub async fn get_user_id(&self, username: impl Into<String>) -> Result<Uuid, DocuBaseError> {
        let username = username.into();
        if !self.user_exists(LookupField::Username(&username)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        let user = sqlx::query!(
            r#"
            SELECT id FROM users WHERE username = $1
        "#,
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user.id)
    }

    async fn username_and_password_match(
        &self,
        username: &str,
        password: &str,
    ) -> Result<bool, DocuBaseError> {
        Ok(sqlx::query!(
            r#"
                        SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 AND password = $2)
                    "#,
            username,
            password
        )
        .fetch_one(&self.pool)
        .await?
        .exists
        .unwrap_or(false))
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserExist |
    /// DocuBaseError::PasswordHashingError |
    /// DocuBaseError::SqlxError
    /// ```
    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<CreateResponse, DocuBaseError> {
        if self.user_exists(LookupField::Username(username)).await? {
            return Err(DocuBaseError::UserExist);
        }

        if self.user_exists(LookupField::Email(email)).await? {
            return Err(DocuBaseError::UserExist);
        }

        let hashed_password = utils::hash_password(password)?;

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

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::PasswordHashingError |
    /// DocuBaseError::UsernameOrPasswordAreWrong |
    /// DocuBaseError::SqlxErron
    /// ```
    ///
    pub async fn delete_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<DeleteResponse, DocuBaseError> {
        if !self.user_exists(LookupField::Username(username)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        let hashed_password = utils::hash_password(password)?;

        if !self
            .username_and_password_match(&hashed_password, password)
            .await?
        {
            return Err(DocuBaseError::UsernameOrPasswordAreWrong);
        }

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
    /// # Errors
    /// ```rust
    /// DocuBaseError::SqlxError(sqlx::Error)
    /// ```
    ///
    /// it will return a query error if the query failed
    async fn project_exists(&self, id: &Uuid) -> Result<bool, DocuBaseError> {
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

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::SqlxError(sqlx::Error)
    /// ```
    async fn project_title_exists(
        &self,
        title: &str,
        user_id: &Uuid,
    ) -> Result<bool, DocuBaseError> {
        let user_projects = self.get_user_projects(user_id).await?;

        Ok(user_projects.iter().any(|project| project.title.eq(title)))
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::SqlxError
    /// ```
    pub async fn get_user_projects(&self, user_id: &Uuid) -> Result<Vec<Project>, DocuBaseError> {
        if !self.user_exists(LookupField::Id(user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
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
                user_id: *user_id,
                project_id: record.id,
                title: record.title,
            })
            .collect::<Vec<_>>())
    }

    async fn project_belongs_to_user(
        &self,
        project_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<bool, DocuBaseError> {
        Ok(sqlx::query!(
            r#"
                    SELECT EXISTS(SELECT 1 FROM projects WHERE id = $1 AND user_id = $2)
                "#,
            project_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?
        .exists
        .unwrap_or(false))
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::ProjectDoesNotExist |
    /// DocuBaseError::UserDoesNotOwnProject |
    /// DocuBaseError::SqlxError
    /// ```
    pub async fn get_project(
        &self,
        project_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<Project, DocuBaseError> {
        if !self.user_exists(LookupField::Id(user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        if !self.project_exists(project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExist);
        }

        if !self.project_belongs_to_user(project_id, user_id).await? {
            return Err(DocuBaseError::UserDoesNotOwnProject);
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
            project_id: *project_id,
            user_id: project.user_id,
            title: project.title,
        })
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::ProjectDoesNotExist |
    /// DocuBaseError::UserDoesNotOwnProject |
    /// DocuBaseError::SqlxError
    /// ```
    pub async fn get_project_state(
        &self,
        project_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<ReportType, DocuBaseError> {
        if !self.user_exists(LookupField::Id(user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        if !self.project_exists(project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExist);
        }

        if !self.project_belongs_to_user(project_id, user_id).await? {
            return Err(DocuBaseError::UserDoesNotOwnProject);
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

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::ProjectExist |
    /// DocuBaseError::SqlxError
    /// ```
    pub async fn create_project(
        &self,
        user_id: &Uuid,
        title: &str,
    ) -> Result<CreateResponse, DocuBaseError> {
        if !self.user_exists(LookupField::Id(user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        if self.project_title_exists(title, user_id).await? {
            return Err(DocuBaseError::ProjectExist);
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

    /// # Errors
    /// ```rust
    /// DocuBaseError::ProjectDoesNotExist |
    /// DocuBaseError::UserDoesNotOwnProject |
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::SqlxError
    /// ```
    pub async fn delete_project(
        &self,
        project_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<DeleteResponse, DocuBaseError> {
        if !self.user_exists(LookupField::Id(user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        if !self.project_exists(project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExist);
        }

        if !self.project_belongs_to_user(project_id, user_id).await? {
            return Err(DocuBaseError::UserDoesNotOwnProject);
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
    /// # Errors
    /// ```rust
    /// DocuBaseError::SqlxError(sqlx::Error)
    /// ```
    async fn report_exists(&self, id: &Uuid) -> Result<bool, DocuBaseError> {
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

    /// # Errors
    /// ```rust
    /// DocuBaseError::SqlxError(sqlx::Error)
    /// ```
    async fn report_exists_with_type(
        &self,
        report: &str,
        project_id: &Uuid,
    ) -> Result<bool, DocuBaseError> {
        let report = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM reports WHERE type = $1 AND project_id = $2)
           "#,
            report,
            project_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(report.exists.unwrap_or(false))
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::SqlxError(sqlx::Error)
    /// ```
    pub async fn report_type_exists(
        &self,
        project_id: &Uuid,
        report_type: ReportType,
    ) -> Result<bool, DocuBaseError> {
        let report = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM reports WHERE project_id = $1 AND type = $2)
           "#,
            project_id,
            report_type.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(report.exists.unwrap_or(false))
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::SqlxError(sqlx::Error)
    /// DocuBaseError::ReportExist
    /// ```
    pub async fn check_report_type_exists(
        &self,
        project_id: &Uuid,
        report_type: ReportType,
    ) -> Result<bool, DocuBaseError> {
        let report = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM reports WHERE project_id = $1 AND type = $2)
           "#,
            project_id,
            report_type.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        report.exists.ok_or(DocuBaseError::ReportExist)
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::SqlxError(sqlx::Error)
    /// ```
    async fn report_belongs_to_project(
        &self,
        project_id: &Uuid,
        report_id: &Uuid,
    ) -> Result<bool, DocuBaseError> {
        Ok(sqlx::query!(
            r#"
                    SELECT EXISTS(SELECT 1 FROM reports WHERE id = $1 AND project_id = $2)
                "#,
            report_id,
            project_id
        )
        .fetch_one(&self.pool)
        .await?
        .exists
        .unwrap_or(false))
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::ProjectDoesNotExist |
    /// DocuBaseError::UserDoesNotOwnProject |
    /// DocuBaseError::ReportExist |
    /// DocuBaseError::SqlxError
    /// ```
    pub async fn create_report<R: ReportMetadata>(
        &self,
        project_id: &Uuid,
        user_id: &Uuid,
        report: &R,
    ) -> Result<CreateResponse, DocuBaseError> {
        if !self.user_exists(LookupField::Id(user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        if !self.project_exists(project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExist);
        }

        if !self.project_belongs_to_user(project_id, user_id).await? {
            return Err(DocuBaseError::UserDoesNotOwnProject);
        }

        let report_type = report.report_type();

        if self.report_type_exists(project_id, report_type).await? {
            return Err(DocuBaseError::ReportExist);
        }

        let report = sqlx::query!(
            r#"
            INSERT INTO reports (project_id, type, content)
            VALUES ($1, $2, $3)
            RETURNING id, created_at
            "#,
            project_id,
            report_type.to_string(),
            report.content(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CreateResponse {
            id: report.id,
            created_at: report.created_at,
        })
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::ProjectDoesNotExist |
    /// DocuBaseError::UserDoesNotOwnProject |
    /// DocuBaseError::ReportDoesNotExist |
    /// DocuBaseError::SqlxError
    /// ```
    pub async fn update_report<R: ReportMetadata>(
        &self,
        project_id: &Uuid,
        user_id: &Uuid,
        report: &R,
    ) -> Result<CreateResponse, DocuBaseError> {
        if !self.user_exists(LookupField::Id(user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        if !self.project_exists(project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExist);
        }

        if !self.project_belongs_to_user(project_id, user_id).await? {
            return Err(DocuBaseError::UserDoesNotOwnProject);
        }

        let report_type = report.report_type();

        if !self.report_type_exists(project_id, report_type).await? {
            return Err(DocuBaseError::ReportDoesNotExist);
        }

        let report = sqlx::query!(
            r#"
            UPDATE reports SET content = $1 WHERE project_id = $2 AND type = $3
            RETURNING id, created_at
            "#,
            report.content(),
            project_id,
            report_type.to_string(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CreateResponse {
            id: report.id,
            created_at: report.created_at,
        })
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::ProjectDoesNotExist |
    /// DocuBaseError::UserDoesNotOwnProject |
    /// DocuBaseError::ReportDoesNotExist |
    /// DocuBaseError::SqlxError
    /// ```
    pub async fn delete_report_with_type(
        &self,
        report: ReportType,
        project_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<DeleteResponse, DocuBaseError> {
        if !self.user_exists(LookupField::Id(user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        if !self.project_exists(project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExist);
        }

        if !self.project_belongs_to_user(project_id, user_id).await? {
            return Err(DocuBaseError::UserDoesNotOwnProject);
        }

        let report = report.to_string();

        if !self.report_exists_with_type(&report, project_id).await? {
            return Err(DocuBaseError::ReportDoesNotExist);
        }

        let report = sqlx::query!(
            r#"
            DELETE FROM reports WHERE type = $1 AND project_id = $2
            RETURNING id
            "#,
            report,
            project_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(DeleteResponse { id: report.id })
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::ProjectDoesNotExist |
    /// DocuBaseError::UserDoesNotOwnProject |
    /// DocuBaseError::ReportDoesNotExist |
    /// DocuBaseError::ProjectDoesNotOwnReport |
    /// DocuBaseError::SqlxError
    /// ```
    pub async fn delete_report(
        &self,
        report_id: &Uuid,
        project_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<DeleteResponse, DocuBaseError> {
        if !self.user_exists(LookupField::Id(user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        if !self.project_exists(project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExist);
        }

        if !self.project_belongs_to_user(project_id, user_id).await? {
            return Err(DocuBaseError::UserDoesNotOwnProject);
        }

        if !self.report_exists(report_id).await? {
            return Err(DocuBaseError::ReportDoesNotExist);
        }

        if !self
            .report_belongs_to_project(project_id, report_id)
            .await?
        {
            return Err(DocuBaseError::ProjectDoesNotOwnReport);
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

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExist |
    /// DocuBaseError::ProjectDoesNotExist |
    /// DocuBaseError::UserDoesNotOwnProject |
    /// DocuBaseError::ReportDoesNotExist |
    /// DocuBaseError::ProjectDoesNotOwnReport |
    /// DocuBaseError::InvalidReportType |
    /// DocuBaseError::SqlxError
    /// ```
    ///
    pub async fn get_report(
        &self,
        report_id: &Uuid,
        project_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<Report, DocuBaseError> {
        if !self.user_exists(LookupField::Id(user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        if !self.project_exists(project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExist);
        }

        if !self.project_belongs_to_user(project_id, user_id).await? {
            return Err(DocuBaseError::UserDoesNotOwnProject);
        }

        if !self.report_exists(report_id).await? {
            return Err(DocuBaseError::ReportDoesNotExist);
        }

        if !self
            .report_belongs_to_project(project_id, report_id)
            .await?
        {
            return Err(DocuBaseError::ProjectDoesNotOwnReport);
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
            report_id: *report_id,
            r#type: report.r#type.try_into()?,
            content: report.content,
        })
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::ProjectDoesNotExist |
    /// DocuBaseError::InvalidReportType |
    /// DocuBaseError::SqlxError
    /// ```
    ///
    /// converting a String to a ``ReportType``, this should almost always work,
    pub async fn get_project_reports(
        &self,
        project_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<Vec<Report>, DocuBaseError> {
        if !self.user_exists(LookupField::Id(user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExist);
        }

        if !self.project_exists(project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExist);
        }

        if !self.project_belongs_to_user(project_id, user_id).await? {
            return Err(DocuBaseError::UserDoesNotOwnProject);
        }

        let reports = sqlx::query!(
            r#"
                SELECT id, type, content from reports WHERE project_id = $1
            "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        reports
            .into_iter()
            .map(|record| {
                Ok(Report {
                    project_id: *project_id,
                    report_id: record.id,
                    r#type: record.r#type.try_into()?,
                    content: record.content,
                })
            })
            .collect::<Result<Vec<_>, DocuBaseError>>()
    }
}
