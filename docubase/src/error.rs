#[derive(Debug, thiserror::Error)]
pub enum DocuBaseError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("user already exists")]
    UserExists,

    #[error("user does not exists")]
    UserDoesNotExists,
}
