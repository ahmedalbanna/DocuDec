//pub mod auth;
mod constants;
pub mod error;
pub mod models;
pub mod utils;

use constants::DATABASE_URL;
use error::DocuBaseError;
use models::{CreateResponse, DeleteResponse, Project, Report, ReportMetadata, ReportType};
use sqlx::{postgres::PgPool, Pool, Postgres};
use uuid::Uuid;

pub struct DocuBase {
    pool: Pool<Postgres>,
}

enum LookupField<'a> {
    Username(&'a str),
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
        dotenv::dotenv().ok();
        let db_url = &*DATABASE_URL;

        let pool = PgPool::connect(db_url).await?;

        Ok(Self { pool })
    }
}

// user
impl DocuBase {
    /// # Errors
    /// ```rust
    /// DocuBaseError::SqlxError(sqlx::Error)
    /// ```
    ///
    /// it will return a query error if the query failed
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

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExists | DocuBaseError::SqlxError
    /// ```
    ///
    /// it will reutrn an error if the user doesn't exists
    /// or a database query error
    pub async fn get_user_id(&self, username: impl Into<String>) -> Result<Uuid, DocuBaseError> {
        let username = username.into();
        if !self.user_exists(LookupField::Username(&username)).await? {
            return Err(DocuBaseError::UserDoesNotExists);
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

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserExists | DocuBaseError::PasswordHashingError | DocuBaseError::SqlxError
    /// ```
    ///
    /// it will reutrn an error if the user already exists, or couldn't hash the provided
    /// passsword, or a database query error
    pub async fn create_user(
        &self,
        username: impl Into<String>,
        email: impl Into<String>,
        password: &str,
    ) -> Result<CreateResponse, DocuBaseError> {
        let username = username.into();
        if self.user_exists(LookupField::Username(&username)).await? {
            return Err(DocuBaseError::UserExists);
        }

        let hashed_password = utils::hash_password(password)?;

        let user = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING id, created_at
            "#,
            username,
            email.into(),
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
    /// DocuBaseError::UserDoesNotExists | DocuBaseError::PasswordHashingError | DocuBaseError::SqlxError
    /// ```
    ///
    /// it will reutrn an error if the user doesn't exists, or couldn't hash the provided
    /// passsword, or a database query error
    pub async fn delete_user(
        &self,
        username: impl Into<String>,
        password: &str,
    ) -> Result<DeleteResponse, DocuBaseError> {
        let username = username.into();
        if !self.user_exists(LookupField::Username(&username)).await? {
            return Err(DocuBaseError::UserDoesNotExists);
        }

        let hashed_password = utils::hash_password(password)?;

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
    /// DocuBaseError::SqlxError(sqlx::Error)
    /// ```
    ///
    /// it will return a query error if the query failed
    async fn project_title_exists(&self, title: &str, id: &Uuid) -> Result<bool, DocuBaseError> {
        let project = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM projects WHERE id = $1 AND title = $2)
           "#,
            id,
            title
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(project.exists.unwrap_or(false))
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExists | DocuBaseError::SqlxError
    /// ```
    ///
    /// it will reutrn an error if the user doesn't exists,
    /// or a database query error
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

    /// # Errors
    /// ```rust
    /// DocuBaseError::ProjectDoesNotExists | DocuBaseError::SqlxError
    /// ```
    ///
    /// it will return an error if there is not project with the provided ``project_id``,
    /// or a database query error
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

    /// # Errors
    /// ```rust
    /// DocuBaseError::ProjectDoesNotExists | DocuBaseError::SqlxError
    /// ```
    ///
    /// it will return an error if there is not project with the provided ``project_id``,
    /// or a database query error
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

    /// # Errors
    /// ```rust
    /// DocuBaseError::UserDoesNotExists | DocuBaseError::ProjectExists |DocuBaseError::SqlxError
    /// ```
    ///
    /// it will reutrn an error if the user doesn't exists, or if the project title for the user
    /// already exists
    /// or a database query error
    pub async fn create_project(
        &self,
        user_id: Uuid,
        title: impl Into<String>,
    ) -> Result<CreateResponse, DocuBaseError> {
        if !self.user_exists(LookupField::Id(&user_id)).await? {
            return Err(DocuBaseError::UserDoesNotExists);
        }

        let title = title.into();

        if self.project_title_exists(&title, &user_id).await? {
            return Err(DocuBaseError::ProjectExists);
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

    // TODO: make it so it will take the ``user_id``, and matches if the project belongs to the
    // user
    /// # Errors
    /// ```rust
    /// DocuBaseError::ProjectDoesNotExists | DocuBaseError::SqlxError
    /// ```
    ///
    /// it will return an error if there is not project with the provided ``project_id``,
    /// or a database query error
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
    /// # Errors
    /// ```rust
    /// DocuBaseError::SqlxError(sqlx::Error)
    /// ```
    ///
    /// it will return a query error if the query failed
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
    ///
    /// it will return a query error if the query failed
    async fn report_type_exists(
        &self,
        project_id: &Uuid,
        report_type: &str,
    ) -> Result<bool, DocuBaseError> {
        let report = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM reports WHERE project_id = $1 AND type = $2)
           "#,
            project_id,
            report_type
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(report.exists.unwrap_or(false))
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::ProjectDoesNotExists | DocuBaseError::ReportExists | DocuBaseError::SqlxError
    /// ```
    ///
    /// it will reutrn an error if the project doesn't exists, or the report (type) already exists
    /// or a database query error
    pub async fn create_report<R: ReportMetadata>(
        &self,
        project_id: Uuid,
        report: &R,
    ) -> Result<CreateResponse, DocuBaseError> {
        if !self.project_exists(&project_id).await? {
            return Err(DocuBaseError::ProjectDoesNotExists);
        }

        let report_type = report.report_type().to_string();

        if self.report_type_exists(&project_id, &report_type).await? {
            return Err(DocuBaseError::ReportExists);
        }

        let report = sqlx::query!(
            r#"
            INSERT INTO reports (project_id, type, content)
            VALUES ($1, $2, $3)
            RETURNING id, created_at
            "#,
            project_id,
            report_type,
            report.content(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CreateResponse {
            id: report.id,
            created_at: report.created_at,
        })
    }

    // TODO: make it so it will take the ``user_id`` and ``project_id``, and matches if they belong to the
    // user
    /// # Errors
    /// ```rust
    /// DocuBaseError::ReportDoesNotExists | DocuBaseError::SqlxError
    /// ```
    ///
    /// it will return an error if there is not report with the provided ``report_id``,
    /// or a database query error
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

    /// # Errors
    /// ```rust
    /// DocuBaseError::ReportDoesNotExists | DocuBaseError::InvalidReportType | DocuBaseError::SqlxError
    /// ```
    ///
    /// it will return an error if there is no report with the provided ``report_id``,
    /// or couldn't convert a String to a ``ReportType``, this should almost always work,
    /// or a database query error
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
            r#type: report.r#type.try_into()?,
            content: report.content,
        })
    }

    /// # Errors
    /// ```rust
    /// DocuBaseError::ProjectDoesNotExists | DocuBaseError::InvalidReportType | DocuBaseError::SqlxError
    /// ```
    ///
    /// it will return an error if there is no project with the provided ``project_id``,
    /// or couldn't convert a String to a ``ReportType``, this should almost always work,
    /// or a database query error
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

        reports
            .into_iter()
            .map(|record| {
                Ok(Report {
                    project_id,
                    report_id: record.id,
                    r#type: record.r#type.try_into()?,
                    content: record.content,
                })
            })
            .collect::<Result<Vec<_>, DocuBaseError>>()
    }
}
