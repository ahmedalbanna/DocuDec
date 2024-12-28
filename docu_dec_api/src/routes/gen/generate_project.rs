use axum::{response::IntoResponse, routing::post, Router};

pub fn route() -> Router {
    Router::new().route("/generate_project", post(generate_project))
}

async fn generate_project() -> impl IntoResponse {
    // Your code here
    "Hello, World!"
}
