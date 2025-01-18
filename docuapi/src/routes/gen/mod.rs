use std::sync::Arc;

use actix_web::web::{self, service};

pub mod error;

mod r#abstract;
mod additional_instruction;
mod project_appendices;
mod project_conclusion;
mod project_content;
mod project_details;
mod project_info;
mod project_references;
mod project_structure;
mod titles;
mod validators;

pub fn config(config: &mut web::ServiceConfig) {
    let model = Arc::new(docuai::get_model().expect("Make sure the GOOGLE_API_KEY env is set"));

    config.service(
        web::scope("/gen")
            .app_data(web::Data::new(Arc::clone(&model)))
            .service(titles::generate_titles)
            .service(project_info::generate_project_info)
            .service(project_content::generate_project_content)
            .service(project_structure::generate_project_structure)
            .service(additional_instruction::generate_additional_instruction)
            .service(r#abstract::generate_abstraction)
            .service(project_details::generate_project_details)
            .service(project_conclusion::generate_project_conclusion)
            .service(project_references::generate_project_references)
            .service(project_appendices::generate_project_appendices),
    );
}
