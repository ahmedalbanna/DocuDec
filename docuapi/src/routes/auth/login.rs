use std::sync::Arc;

use actix_web::{
    cookie::{time::Duration, Cookie, SameSite},
    web::{self, Json},
    HttpResponse,
};
use docubase::DocuBase;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

use super::error::AuthError;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Login {
    username: String,
    password: String,
}

/// # Success
/// ```rust
/// {
///     "jwt_token": "23fkjnd-sdkcnas-akajshcs",
/// }
/// ```
/// # Errors
/// ```rust
/// AuthError::UserDoesNotExists => (404, "the user doesn't exists")
/// AuthError::InternalError => (500, "internal error")
/// AuthError::JwtError => (417, "jwt creation error") // when we can't generate a jwt token out of
/// the user_id
/// ```
///
#[instrument(skip(db))]
#[actix_web::post("/login")]
pub async fn login(
    db: web::Data<Arc<DocuBase>>,
    payload: Json<Login>,
) -> Result<HttpResponse, AuthError> {
    let user_id = db.get_user_id(&payload.username).await?;
    let token = docubase::utils::generate_jwt(&user_id.to_string())?;

    let cookie = Cookie::build(crate::utils::constants::JWT_COOKIE, token.clone())
        .path("/") // The cookie is accessible across all paths on the domain
        .max_age(Duration::days(7)) // Set the cookie to expire in 7 days
        .http_only(true) // Prevent JavaScript access to the cookie
        .same_site(SameSite::Strict) // Prevent CSRF attacks
        .secure(false) // TODO: change it in production
        .finish();

    debug!("login successful for user: {}", payload.username);

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({ "jwt_token": token })))
}
