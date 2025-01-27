use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, thiserror::Error)]
pub enum MiddlewareError {
    #[error("you can't access the endpoint without the jwt cookie")]
    NoJwtTokenProvided,

    #[error("the jwt token is invalid")]
    InvalidJwt,
}

impl MiddlewareError {
    pub const fn to_status_code(&self) -> StatusCode {
        use MiddlewareError as E;
        match self {
            E::NoJwtTokenProvided => StatusCode::FORBIDDEN,
            E::InvalidJwt => StatusCode::BAD_REQUEST,
        }
    }

    pub fn to_response(&self) -> MiddlewareErrorResponse {
        MiddlewareErrorResponse {
            error: self.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareErrorResponse {
    error: String,
}

impl actix_web::Responder for MiddlewareError {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let (status, body) = (self.to_status_code(), self.to_response());

        HttpResponse::build(status)
            .content_type("application/json")
            .json(body)
    }
}

impl actix_web::ResponseError for MiddlewareError {
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
