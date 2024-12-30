use std::sync::Arc;

use axum::{routing::post, Router};

pub mod error;
mod gen_project_info;
mod gen_title;

pub async fn routes() -> Router {
    let model = Arc::new(docuai::get_model().expect("Make sure the GOOGLE_API_KEY env is set"));

    Router::new()
        .route("/gen-titles", post(gen_title::generate_titles))
        .route(
            "/gen-project-info",
            post(gen_project_info::generate_project_info),
        )
        .with_state(model)
}
