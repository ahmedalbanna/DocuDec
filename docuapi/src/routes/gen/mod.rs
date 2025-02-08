//! contains the generation routers
//!
//! - alternative titles
//! - project info
//! - project content
//! - project structure
//! - additional instruction
//! - project abstract
//! - project conclusion
//! - project references
//! - project appendices

use std::sync::Arc;

use crate::mw::my_middleware;
use actix_web::{middleware::from_fn, web};
use docuai::{
    docu_requests::{context::ProcessRequest, RequestSchema},
    DocuError, DocuRequest,
};
use error::GenError;
use google_generative_ai_rs::v1::api::Client;
use serde::de::DeserializeOwned;
use tracing::error;

pub use payload_getter::ProvidesReport;

pub mod error;

pub mod additional_instruction;
pub mod payload_getter;
pub mod project_abstract;
pub mod project_appendices;
pub mod project_conclusion;
pub mod project_content;
pub mod project_info;
pub mod project_references;
pub mod project_structure;
pub mod titles;
pub mod validators;

/// # Panics
/// if would panic if the `GOOGLE_API_KEY` is not set
pub fn config(config: &mut web::ServiceConfig) {
    let model = Arc::new(docuai::get_model().expect("Make sure the GOOGLE_API_KEY env is set"));

    config.service(
        web::scope("/gen")
            .app_data(web::Data::new(Arc::clone(&model)))
            .wrap(from_fn(my_middleware))
            .service(titles::generate_titles)
            .service(project_info::generate_project_info)
            .service(project_content::generate_project_content)
            .service(project_structure::generate_project_structure)
            .service(additional_instruction::generate_additional_instruction)
            .service(project_abstract::generate_project_abstract)
            .service(project_conclusion::generate_project_conclusion)
            .service(project_references::generate_project_references)
            .service(project_appendices::generate_project_appendices),
    );
}

/// # Errors
/// ```rust
/// GenError::ApiRequestError => (500 , "Internal AI error: { String }"),
/// GenError::SyntaxError => (400, "Syntax error: { String }")
/// ```
///
/// # Panics
///
/// will panic if there is no text in the request
async fn generate_report<R>(model: &Client, request: &DocuRequest<'_, R>) -> Result<R, GenError>
where
    R: DeserializeOwned + RequestSchema + ProcessRequest,
{
    match docuai::request(model, request, crate::utils::constants::API_TIMEOUT).await {
        Ok(report) => Ok(report),

        Err(error) => {
            error!("generation failed: {:?}", error);
            match error {
                DocuError::ApiRequest(ge) => {
                    error!(
                        "AI request failed. Error: {}, Status: {:?}",
                        ge.message, ge.code
                    );

                    // return ApiRequestError
                    Err(ge.into())
                }
                DocuError::JsonConversion(jc) => {
                    error!("Json conversion failed, error: {jc:?}");
                    // return SyntaxError
                    Err(jc.into())
                }
                DocuError::NoTextFoundInResponse => {
                    panic!("API Error, the response does not have a text, this should not happen")
                }
            }
        }
    }
}

enum UpdateAction {
    Yes,
    YesIfExist,
    No,
}
fn update_action(
    update: Option<bool>,
    update_if_exist: Option<bool>,
) -> Result<UpdateAction, GenError> {
    if update.is_some_and(|b| b) && update_if_exist.is_some_and(|b| b) {
        return Err(GenError::UpdateMisuse);
    }

    if update.is_some_and(|b| b) {
        return Ok(UpdateAction::Yes);
    }

    if update_if_exist.is_some_and(|b| b) {
        return Ok(UpdateAction::YesIfExist);
    }

    Ok(UpdateAction::No)
}
