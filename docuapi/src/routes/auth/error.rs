use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use docubase::error::DocuBaseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, thiserror::Error)]
pub enum AuthError {
    #[error("the user already has an account")]
    UserIsAlreadySignedUp,

    #[error("the user doesn't exists")]
    UserDoesNotExists,

    #[error("internal error")]
    InternalError,

    #[error("jwt creation error")]
    JwtError,

    /// TODO: remove this
    #[error("unknown error")]
    Unknown,
}

impl AuthError {
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            AuthError::UserIsAlreadySignedUp => StatusCode::FORBIDDEN,
            AuthError::UserDoesNotExists => StatusCode::NOT_FOUND,
            AuthError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::JwtError => StatusCode::EXPECTATION_FAILED,
            AuthError::Unknown => StatusCode::NO_CONTENT,
        }
    }

    pub fn to_response(&self) -> AuthErrorResponse {
        AuthErrorResponse {
            error: self.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthErrorResponse {
    error: String,
}

impl actix_web::Responder for AuthError {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let (status, body) = (self.to_status_code(), self.to_response());

        HttpResponse::build(status)
            .content_type("application/json")
            .json(body)
    }
}

impl actix_web::ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        self.to_status_code()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status = self.status_code();

        HttpResponse::build(status)
            .content_type("application/json")
            .json(self.to_response())
    }
}

impl From<DocuBaseError> for AuthError {
    fn from(value: DocuBaseError) -> Self {
        match value {
            DocuBaseError::UserExists => Self::UserIsAlreadySignedUp,
            DocuBaseError::UserDoesNotExists => Self::UserDoesNotExists,
            DocuBaseError::SqlxError(_) => Self::InternalError,
            DocuBaseError::JwtError(_) => Self::JwtError,
            _ => Self::Unknown,
        }
    }
}
