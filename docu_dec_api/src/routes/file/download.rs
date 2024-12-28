use axum::{response::IntoResponse, routing::get, Router};

pub fn route() -> Router {
    Router::new().route("/download", get(download))
}

async fn download() -> impl IntoResponse {
    // Your code here
    "Hello, World!"
}
