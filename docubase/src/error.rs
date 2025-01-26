#[derive(Debug, thiserror::Error)]
pub enum DocuBaseError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("user already exists")]
    UserExists,

    #[error("user does not exists")]
    UserDoesNotExists,

    #[error("project does ot exists")]
    ProjectDoesNotExists,

    #[error("project alredy exists")]
    ProjectExists,

    #[error("report does ot exists")]
    ReportDoesNotExists,

    #[error("report (type) alredy exists")]
    ReportExists,

    #[error("project state is not found, perhabs it's a new project")]
    ProjectStateNotFound,

    #[error("the report type or project state is invalid, got: {0}")]
    InvalidReportType(String),

    #[error("the jwt token is expired")]
    ExpiredToken,

    #[error("jwt error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    // couldn't put the argon2 directly to here, because of the as_dyn_mut or something
    #[error("couldn't hash the password: {0}")]
    PasswordHashingError(String),
}
