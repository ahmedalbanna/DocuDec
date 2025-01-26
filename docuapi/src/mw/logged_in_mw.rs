use crate::mw::error::MiddlewareError;

use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};

// FIXME: handle stuff better

/// # Errors
/// ```rust
/// MiddlewareError::NoJwtTokenProvided => (403, "you can't access the endpoint without the jwt cookie")
/// MiddlewareError::InvalidJwt => (400, "the jwt token is invalid")
/// ```
pub async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let jwt_cookie = req
        .cookie(crate::utils::constants::JWT_COOKIE)
        .ok_or(MiddlewareError::NoJwtTokenProvided)?;

    // jwt must be valid before procceding
    docubase::utils::validate_jwt(jwt_cookie.value()).map_err(Into::<MiddlewareError>::into)?;

    next.call(req).await
}
