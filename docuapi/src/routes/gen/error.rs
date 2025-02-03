use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use docubase::{error::DocuBaseError, models::ReportType};
use google_generative_ai_rs::v1::errors::GoogleAPIError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenError {
    #[error("Internal AI error: {0}")]
    AiRequestError(String),

    // TODO: remove this
    #[error("Unknown error")]
    UnknownError,

    #[error("Invalid title: {0}")]
    InvalidTitle(String),

    #[error("Syntax error: {0}")]
    SyntaxError(String),

    #[error("Missing content: {0}")]
    MissingContent(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("missing reports: {0}")]
    MissingReports(ReportType),

    #[error("you can't use update and update_if_exist at the same time")]
    UpdateMisuse,

    #[error("{0}")]
    DBError(#[from] DocuBaseError),
}

impl GenError {
    #[must_use]
    const fn to_status_code(&self) -> StatusCode {
        match self {
            GenError::ValidationError(_)
            | GenError::MissingContent(_)
            | GenError::InvalidTitle(_)
            | GenError::SyntaxError(_)
            | GenError::UpdateMisuse => StatusCode::BAD_REQUEST,

            GenError::AiRequestError(_) | GenError::UnknownError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }

            GenError::MissingReports(_) => StatusCode::NOT_FOUND,

            // Database errors
            GenError::DBError(error) => match error {
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
    fn to_response(&self) -> GenErrorResponse {
        GenErrorResponse {
            error: self.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct GenErrorResponse {
    pub error: String,
}

impl actix_web::Responder for GenError {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let (status, body) = (self.to_status_code(), self.to_response());

        HttpResponse::build(status)
            .content_type("application/json")
            .json(body)
    }
}

impl actix_web::ResponseError for GenError {
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

impl From<serde_json::Error> for GenError {
    fn from(err: serde_json::Error) -> Self {
        GenError::SyntaxError(err.to_string())
    }
}

impl From<GoogleAPIError> for GenError {
    fn from(err: GoogleAPIError) -> Self {
        GenError::AiRequestError(err.message)
    }
}
