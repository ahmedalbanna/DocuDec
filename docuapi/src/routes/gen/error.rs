use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use google_generative_ai_rs::v1::errors::GoogleAPIError;
use serde::Serialize;

/// a helper function
pub fn map_ai_error(error: GoogleAPIError) -> GenError {
    GenError::AiRequestError(format!(
        "Couldn't complete the AI request, error: {:#?}",
        error.message
    ))
}

#[derive(Debug)]
pub enum GenError {
    AiRequestError(String),
    UnknownError,
    EmptyTitle,
}

#[derive(Serialize)]
pub struct GenErrorResponse {
    pub error: String,
}

impl std::error::Error for GenError {}

impl std::fmt::Display for GenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AiRequestError(message) => write!(f, "AI Request Error: {}", message),
            Self::EmptyTitle => write!(f, "Title is empty"),
            Self::UnknownError => write!(f, "Unknown Error"),
        }
    }
}

impl IntoResponse for GenError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            Self::AiRequestError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                GenErrorResponse { error: message },
            ),
            Self::EmptyTitle => (
                StatusCode::BAD_REQUEST,
                GenErrorResponse {
                    error: "The title cannot be empty".to_string(),
                },
            ),
            Self::UnknownError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                GenErrorResponse {
                    error: "Unknown Error".to_string(),
                },
            ),
        };

        (status, Json(body)).into_response()
    }
}
