use axum::{
    extract::rejection::{JsonRejection, JsonSyntaxError, MissingJsonContentType},
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

pub fn map_json_rejection<J>(err: JsonRejection) -> GenError
where
    J: Serialize + Default,
{
    match err {
        JsonRejection::JsonSyntaxError(e) => GenError::SyntaxError(e),
        JsonRejection::JsonDataError(e) => GenError::DataError(
            e.to_string(),
            serde_json::to_string_pretty(&J::default())
                .unwrap_or("Failed to get the expected value".into()),
        ),
        JsonRejection::MissingJsonContentType(e) => GenError::MissingContent(e),
        _ => GenError::UnknownError,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GenError {
    #[error("Internal AI error: {0}")]
    AiRequestError(String),

    #[error("Unknown error")]
    UnknownError,

    #[error("The title was not provided")]
    EmptyTitle,

    #[error("Json syntax error, make sure you type it right, error: {0}")]
    SyntaxError(JsonSyntaxError),

    #[error("Json data error, error: {0}")]
    DataError(String, String),

    #[error("Json missing content type, error: {0}")]
    MissingContent(MissingJsonContentType),
}

#[derive(Serialize, Default)]
pub struct GenErrorResponse {
    pub error: String,
    pub expected: String,
}

impl IntoResponse for GenError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            Self::AiRequestError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                GenErrorResponse {
                    error: message,
                    ..Default::default()
                },
            ),

            Self::EmptyTitle => (
                StatusCode::BAD_REQUEST,
                GenErrorResponse {
                    error: self.to_string(),
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

            Self::SyntaxError(_) => (
                StatusCode::BAD_REQUEST,
                GenErrorResponse {
                    error: self.to_string(),
                    ..Default::default()
                },
            ),

            Self::DataError(_, ref expected) => (
                StatusCode::BAD_REQUEST,
                GenErrorResponse {
                    error: self.to_string(),
                    expected: expected.to_owned(),
                },
            ),

            Self::MissingContent(_) => (
                StatusCode::BAD_REQUEST,
                GenErrorResponse {
                    error: self.to_string(),
                    ..Default::default()
                },
            ),
        };

        (status, Json(body)).into_response()
    }
}
