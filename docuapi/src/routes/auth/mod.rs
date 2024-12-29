use axum::{routing::post, Router};

mod login;
mod signup;

pub async fn routes() -> Router {
    Router::new()
        .route("/login", post(login::login))
        .route("/signup", post(signup::signup))
}
