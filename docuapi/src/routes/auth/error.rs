use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use docubase::error::DocuBaseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("{0}")]
    DBError(#[from] docubase::error::DocuBaseError),
}

impl AuthError {
    #[must_use]
    pub const fn to_status_code(&self) -> StatusCode {
        match self {
            // Database errors
            AuthError::DBError(error) => match error {
                // Not Found
                DocuBaseError::UserDoesNotExist
                | DocuBaseError::ProjectDoesNotExist
                | DocuBaseError::ReportDoesNotExist
                | DocuBaseError::ProjectStateNotFound => StatusCode::NOT_FOUND,

                // Conflict
                DocuBaseError::UserExist
                | DocuBaseError::ProjectExist
                | DocuBaseError::ReportExist => StatusCode::CONFLICT,

                // Forbidden
                DocuBaseError::UserDoesNotOwnProject | DocuBaseError::ProjectDoesNotOwnReport => {
                    StatusCode::FORBIDDEN
                }

                // Unauthorized
                DocuBaseError::UsernameOrPasswordAreWrong | DocuBaseError::ExpiredToken => {
                    StatusCode::UNAUTHORIZED
                }

                // Bad Request
                DocuBaseError::InvalidReportType(_) => StatusCode::BAD_REQUEST,

                // Internal Server Error
                DocuBaseError::SqlxError(_)
                | DocuBaseError::JwtError(_)
                | DocuBaseError::PasswordHashingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    #[must_use]
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
