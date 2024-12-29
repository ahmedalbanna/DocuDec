use axum::{routing::get, Router};

pub mod download;

pub async fn route() -> Router {
    Router::new().route("/download", get(download::download))
}
