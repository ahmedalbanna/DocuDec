pub mod schema_builder;

pub mod requests;
pub mod responses;
pub mod schemas;
pub mod sysinsts;

use google_generative_ai_rs::v1::{
    api::PostResult,
    gemini::request::{GenerationConfig, Request},
};
use serde::de::DeserializeOwned;
use std::{default::Default, ops::Deref};

use crate::error::DocuError;

pub enum DocuRequest {
    AlterTitles,
    ProjectInfo,
}

#[derive(Debug, Default)]
pub struct DocuRequestInfo {
    title: Option<String>,
}

impl DocuRequestInfo {
    pub fn title(title: String) -> Self {
        Self {
            title: Some(title),
            ..Default::default()
        }
    }

    pub fn get_title(&self) -> Option<String> {
        self.title.clone()
    }
}

impl DocuRequest {
    pub fn to_config(&self) -> GenerationConfig {
        match self {
            Self::AlterTitles => schemas::alter_titles::ALTER_TITLES.deref().clone(),
            Self::ProjectInfo => schemas::project_info::PROJECT_INFO.deref().clone(),
        }
    }

    pub fn to_system_instruction(&self) -> String {
        match self {
            Self::AlterTitles => {
                sysinsts::alter_titles::ALTER_TITLES_SYSTEM_INSTRUCTION.to_string()
            }
            Self::ProjectInfo => {
                sysinsts::project_info::PROJECT_INFO_SYSTEM_INSTRUCTION.to_string()
            }
        }
    }

    pub async fn to_response<R>(&self, post_result: PostResult) -> Result<R, DocuError>
    where
        R: DeserializeOwned,
    {
        match self {
            Self::AlterTitles => {
                let text = crate::get_text(post_result)
                    .await
                    .ok_or(DocuError::NoTextFoundInResponse)?;

                let parsed_json = serde_json::from_str(&text)?;
                Ok(parsed_json)
            }

            Self::ProjectInfo => {
                let text = crate::get_text(post_result)
                    .await
                    .ok_or(DocuError::NoTextFoundInResponse)?;

                let parsed_json = serde_json::from_str(&text)?;
                Ok(parsed_json)
            }
        }
    }
    pub fn to_request(&self, info: DocuRequestInfo) -> Result<Request, DocuError> {
        match self {
            Self::AlterTitles => {
                let orig_title = info.get_title().ok_or_else(|| {
                    DocuError::TitleNotFound(
                        "Couldn't get title from the DocuRequestInfo while converting an AlterTitles to a Requests".to_string(),
                    )
                })?;

                Ok(requests::alter_titles::get_request(orig_title))
            }
            Self::ProjectInfo => {
                let title = info.get_title().ok_or_else(|| {
                    DocuError::TitleNotFound(
                        "Couldn't get the title from the DocuRequestInfo while converting a ProjectInfo to a Request".to_string()
                    )
                })?;

                Ok(requests::project_info::get_request(title))
            }
        }
    }
}
