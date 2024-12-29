pub mod schema_builder;

pub mod requests;
pub mod schemas;
pub mod sysinst;

use google_generative_ai_rs::v1::gemini::request::{GenerationConfig, Request};
use std::{default::Default, ops::Deref};

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
            Self::AlterTitles => sysinst::alter_titles::ALTER_TITLES_SYSTEM_INSTRUCTION.to_string(),
            Self::ProjectInfo => sysinst::project_info::PROJECT_INFO_SYSTEM_INSTRUCTION.to_string(),
        }
    }

    pub fn to_request(&self, info: DocuRequestInfo) -> Request {
        match self {
            Self::AlterTitles => {
                let orig_title = info.get_title().unwrap();
                requests::alter_titles::get_request(orig_title)
            }
            Self::ProjectInfo => {
                let title = info.get_title().unwrap();
                requests::project_info::get_request(title)
            }
        }
    }
}
