use actix_web::{
    error::{InternalError, JsonPayloadError},
    http::StatusCode,
    HttpRequest, HttpResponse,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonErrorResponse {
    error: String,
    error_type: String,
    field: Option<String>,
    details: Option<String>,
}

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> actix_web::Error {
    let (status_code, error_response) = match &err {
        JsonPayloadError::ContentType => (
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            JsonErrorResponse {
                error: "Incorrect Content-Type".to_string(),

                error_type: "CONTENT_TYPE_ERROR".to_string(),

                field: None,

                details: Some("Expected application/json".to_string()),
            },
        ),

        JsonPayloadError::Deserialize(json_err) => {
            let error_details = json_err.to_string();

            let field_match = error_details
                .split("missing field `")
                .nth(1)
                .and_then(|s| s.split('`').next())
                .map(ToString::to_string);

            (
                StatusCode::BAD_REQUEST,
                JsonErrorResponse {
                    error: "JSON Deserialization Error".to_string(),

                    error_type: "DESERIALIZE_ERROR".to_string(),

                    field: field_match,

                    details: Some(error_details),
                },
            )
        }

        _ => (
            StatusCode::BAD_REQUEST,
            JsonErrorResponse {
                error: "Unknown JSON Error".to_string(),

                error_type: "UNKNOWN_ERROR".to_string(),

                field: None,

                details: Some(err.to_string()),
            },
        ),
    };

    // Create a response with the specific status code

    InternalError::from_response(
        err,
        HttpResponse::build(status_code)
            .content_type("application/json")
            .json(error_response),
    )
    .into()
}
