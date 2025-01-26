use actix_web::{
    body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder, ResponseError,
};
use docubase::error::DocuBaseError;
use google_generative_ai_rs::v1::errors::GoogleAPIError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A helper function to map Google API errors to ``GenError``
pub fn map_ai_error(error: &GoogleAPIError) -> GenError {
    GenError::AiRequestError(format!(
        "Couldn't complete the AI request, error: {:#?}",
        error.message
    ))
}

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

    #[error("project alredy exists")]
    ProjectExists,

    #[error("project doesn't exists")]
    ProjectDoesNotExists,

    #[error("report alredy exists")]
    ReportExists,

    #[error("internal database error")]
    InternalDBError,

    #[error("jwt error")]
    JwtError,

    #[error("the user doesn't exists")]
    UserDoesNotExists,
}

impl GenError {
    fn to_status_code(&self) -> StatusCode {
        match self {
            GenError::ProjectExists | GenError::ReportExists => StatusCode::CONFLICT,
            GenError::InternalDBError | GenError::UnknownError | GenError::AiRequestError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            GenError::UserDoesNotExists => StatusCode::NOT_FOUND,
            GenError::SyntaxError(_)
            | GenError::InvalidTitle(_)
            | GenError::MissingContent(_)
            | GenError::ValidationError(_)
            | GenError::JwtError
            | GenError::ProjectDoesNotExists => StatusCode::BAD_REQUEST,
        }
    }

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

// Optional: Conversion from GoogleAPIError
impl From<&GoogleAPIError> for GenError {
    fn from(err: &GoogleAPIError) -> Self {
        map_ai_error(err)
    }
}

impl From<DocuBaseError> for GenError {
    fn from(value: DocuBaseError) -> Self {
        use DocuBaseError as DE;
        match value {
            DE::ProjectExists => Self::ProjectExists,
            DE::ReportExists => Self::ReportExists,
            DE::SqlxError(_) => Self::InternalDBError,
            DE::JwtError(_) => Self::JwtError,
            DE::UserDoesNotExists => Self::UserDoesNotExists,
            DE::ProjectDoesNotExists => Self::ProjectDoesNotExists,
            _ => Self::UnknownError,
        }
    }
}
