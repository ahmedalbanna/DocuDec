use axum::{response::IntoResponse, routing::post, Router};

pub fn route() -> Router {
    Router::new().route("/signup", post(signup))
}

async fn signup() -> impl IntoResponse {
    // Your code here
    "Hello, World!"
}
