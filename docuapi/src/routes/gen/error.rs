use actix_web::{
    body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder, ResponseError,
};
use google_generative_ai_rs::v1::errors::GoogleAPIError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A helper function to map Google API errors to GenError
pub fn map_ai_error(error: GoogleAPIError) -> GenError {
    GenError::AiRequestError(format!(
        "Couldn't complete the AI request, error: {:#?}",
        error.message
    ))
}

#[derive(Debug, Error)]
pub enum GenError {
    #[error("Internal AI error: {0}")]
    AiRequestError(String),

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
}

#[derive(Serialize, Deserialize, Default)]
pub struct GenErrorResponse {
    pub error: String,

    #[serde(default)]
    pub expected: String,
}

// Implementing Responder for GenError
impl Responder for GenError {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let (status, body) = match self {
            Self::AiRequestError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                GenErrorResponse {
                    error: message,
                    ..Default::default()
                },
            ),

            Self::InvalidTitle(message) => (
                StatusCode::BAD_REQUEST,
                GenErrorResponse {
                    error: message,
                    ..Default::default()
                },
            ),

            Self::ValidationError(message) => (
                StatusCode::BAD_REQUEST,
                GenErrorResponse {
                    error: message,
                    ..Default::default()
                },
            ),

            Self::UnknownError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                GenErrorResponse {
                    error: self.to_string(),
                    ..Default::default()
                },
            ),

            Self::SyntaxError(message) => (
                StatusCode::BAD_REQUEST,
                GenErrorResponse {
                    error: message,
                    ..Default::default()
                },
            ),

            Self::MissingContent(message) => (
                StatusCode::BAD_REQUEST,
                GenErrorResponse {
                    error: message,
                    ..Default::default()
                },
            ),
        };

        HttpResponse::build(status)
            .content_type("application/json")
            .json(body)
    }
}

// Implementing ResponseError for GenError
impl ResponseError for GenError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            GenError::AiRequestError(message) => HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(GenErrorResponse {
                    error: message.clone(),
                    ..Default::default()
                }),

            GenError::InvalidTitle(message) => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(GenErrorResponse {
                    error: message.clone(),
                    ..Default::default()
                }),

            GenError::ValidationError(message) => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(GenErrorResponse {
                    error: message.clone(),
                    ..Default::default()
                }),

            GenError::UnknownError => HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(GenErrorResponse {
                    error: self.to_string(),
                    ..Default::default()
                }),

            GenError::SyntaxError(message) => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(GenErrorResponse {
                    error: message.clone(),
                    ..Default::default()
                }),

            GenError::MissingContent(message) => HttpResponse::BadRequest()
                .content_type("application/json")
                .json(GenErrorResponse {
                    error: message.clone(),
                    ..Default::default()
                }),
        }
    }
    fn status_code(&self) -> StatusCode {
        match self {
            GenError::AiRequestError(_) | GenError::UnknownError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            GenError::InvalidTitle(_) | GenError::SyntaxError(_) | GenError::MissingContent(_) => {
                StatusCode::BAD_REQUEST
            }
            GenError::ValidationError(_) => StatusCode::NON_AUTHORITATIVE_INFORMATION,
        }
    }
}

// Conversion implementations
impl From<std::io::Error> for GenError {
    fn from(err: std::io::Error) -> Self {
        GenError::AiRequestError(err.to_string())
    }
}

impl From<serde_json::Error> for GenError {
    fn from(err: serde_json::Error) -> Self {
        GenError::SyntaxError(err.to_string())
    }
}

// Optional: Additional error conversions
impl From<std::string::FromUtf8Error> for GenError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        GenError::SyntaxError(err.to_string())
    }
}

// Optional: Conversion from GoogleAPIError
impl From<GoogleAPIError> for GenError {
    fn from(err: GoogleAPIError) -> Self {
        map_ai_error(err)
    }
}
