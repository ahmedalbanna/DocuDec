use axum::{response::IntoResponse, routing::post, Router};

pub fn route() -> Router {
    Router::new().route("/login", post(login))
}

async fn login() -> impl IntoResponse {
    // Your code here
    "Hello, World!"
}
