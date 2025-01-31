#[derive(Debug, thiserror::Error)]
pub enum DocuBaseError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("user already exist")]
    UserExist,

    #[error("user does not exist")]
    UserDoesNotExist,

    #[error("user does not own the project")]
    UserDoesNotOwnProject,

    #[error("username or password is wrong")]
    UsernameOrPasswordAreWrong,

    #[error("project does not exist")]
    ProjectDoesNotExist,

    #[error("project already exist")]
    ProjectExist,

    #[error("project does not own the report")]
    ProjectDoesNotOwnReport,

    #[error("report does not exist")]
    ReportDoesNotExist,

    #[error("report (type) already exist")]
    ReportExist,

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
