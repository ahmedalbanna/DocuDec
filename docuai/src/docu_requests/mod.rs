pub mod schema_builder;

pub mod requests;
pub mod responses;
pub mod schema;
pub mod schemas;
pub mod sysinsts;

use google_generative_ai_rs::v1::{
    api::PostResult,
    gemini::request::{GenerationConfig, Request},
};
use responses::project_info::ProjectInfo;
use serde::de::DeserializeOwned;

use crate::error::DocuError;

pub enum DocuRequest<'a> {
    AlterTitles(String),
    ProjectInfo(String),
    ProjectContent(&'a ProjectInfo),
}

impl DocuRequest<'_> {
    pub fn to_config(&self) -> GenerationConfig {
        match self {
            Self::AlterTitles(_) => {
                schema::get_gen_config(schemas::alter_titles::create_alternative_titles_schema())
            }
            Self::ProjectInfo(_) => {
                schema::get_gen_config(schemas::project_info::create_project_info_schema())
            }
            Self::ProjectContent(_) => {
                schema::get_gen_config(schemas::project_content::create_alternative_titles_schema())
            }
        }
    }

    pub fn to_system_instruction(&self) -> String {
        match self {
            Self::AlterTitles(_) => {
                sysinsts::alter_titles::ALTER_TITLES_SYSTEM_INSTRUCTION.to_string()
            }
            Self::ProjectInfo(_) => {
                sysinsts::project_info::PROJECT_INFO_SYSTEM_INSTRUCTION.to_string()
            }
            Self::ProjectContent(_) => {
                sysinsts::project_content::PROJECT_CONTENT_SYSTEM_INSTRUCTION.to_string()
            }
        }
    }

    pub async fn to_response<R>(&self, post_result: PostResult) -> Result<R, DocuError>
    where
        R: DeserializeOwned,
    {
        let text = crate::get_text(post_result)
            .await
            .ok_or(DocuError::NoTextFoundInResponse)?;

        let parsed_json = serde_json::from_str(&text)?;
        Ok(parsed_json)
    }

    pub fn to_request(&self) -> Result<Request, DocuError> {
        match self {
            Self::AlterTitles(orig_title) => Ok(requests::get_request(
                orig_title.to_owned(),
                self.to_config(),
                self.to_system_instruction(),
            )),
            Self::ProjectInfo(chosen_title) => Ok(requests::get_request(
                chosen_title.to_owned(),
                self.to_config(),
                self.to_system_instruction(),
            )),
            Self::ProjectContent(project_info) => {
                let message = serde_json::to_string(*project_info)?;

                Ok(requests::get_request(
                    message,
                    self.to_config(),
                    self.to_system_instruction(),
                ))
            }
        }
    }
}
