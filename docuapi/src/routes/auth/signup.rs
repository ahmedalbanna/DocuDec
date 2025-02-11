use std::sync::Arc;

use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use docubase::DocuBase;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::error::AuthError;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Signup {
    username: String,
    email: String,
    password: String,
}

/// # Request
/// ```json
/// {
///     "username": "",
///     "password": "",
///     "email": ""
/// }
/// ```
/// # Success
/// ```rust
/// {
///     "id": "23fkjnd-sdkcnas-akajshcs",
///     "created_at": 2025-23-1T4:49
/// }
/// ```
/// # Errors
/// ```rust
/// AuthError::UserIsAlreadySignedUp => (403, "the user already has an account")
/// ```
#[instrument(skip(db))]
#[actix_web::post("/signup")]
pub async fn signup(
    db: web::Data<Arc<DocuBase>>,
    payload: Json<Signup>,
) -> Result<HttpResponse, AuthError> {
    let create_response = db
        .create_user(&payload.username, &payload.email, &payload.password)
        .await?;

    Ok(HttpResponse::Ok().json(create_response))
}
