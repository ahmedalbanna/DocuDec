use crate::mw::error::MiddlewareError;

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};

/// # Request
///
/// to get access to the generation/file endpoint, you must send an AUTHORIZATION header with
/// "Bearer JWT_TOKEN_HERE"
///
/// # Errors
/// ```rust
/// MiddlewareError::NoJwtTokenProvided => (403, "you can't access the endpoint without the Authentication header set to the user jwt token")
/// MiddlewareError::InvalidJwt => (400, "the jwt token is invalid")
/// ```
pub async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth_header = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .ok_or(MiddlewareError::NoJwtTokenProvided)?;

    let token = auth_header
        .to_str()
        .map(|s| s.strip_prefix("Bearer "))
        .map_err(|_| MiddlewareError::InvalidJwt)?
        .ok_or(MiddlewareError::InvalidJwt)?;

    // jwt must be valid before procceding
    docubase::utils::validate_jwt(token).map_err(|_| MiddlewareError::InvalidJwt)?;

    next.call(req).await
}
