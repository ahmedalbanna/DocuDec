use std::sync::Arc;

use actix_web::web;

pub mod error;

mod additional_instruction;
mod project_abstract;
mod project_content;
mod project_info;
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
            .service(project_abstract::generate_project_abstraction),
    );
}
