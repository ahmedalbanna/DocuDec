use std::sync::Arc;

use axum::{routing::post, Router};

mod gen_title;

pub async fn routes() -> Router {
    let model = Arc::new(gemini_ai::get_model());

    Router::new()
        .route("/gen-titles", post(gen_title::generate_titles))
        .with_state(model)
}
