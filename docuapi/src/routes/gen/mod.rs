use std::sync::Arc;

use axum::{routing::post, Router};

pub mod error;
mod gen_additional_instruction;
mod gen_project_content;
mod gen_project_info;
mod gen_project_structure;
mod gen_title;

pub async fn routes() -> Router {
    let model = Arc::new(docuai::get_model().expect("Make sure the GOOGLE_API_KEY env is set"));

    Router::new()
        .route("/titles", post(gen_title::generate_titles))
        .route(
            "/project-info",
            post(gen_project_info::generate_project_info),
        )
        .route(
            "/project-content",
            post(gen_project_content::generate_project_content),
        )
        .route(
            "/project-structure",
            post(gen_project_structure::generate_project_structure),
        )
        .route(
            "/additional-instructions",
            post(gen_additional_instruction::generate_additional_instructions),
        )
        .with_state(model)
}
