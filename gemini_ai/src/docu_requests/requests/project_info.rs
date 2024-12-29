use std::ops::Deref;

use google_generative_ai_rs::v1::gemini::{
    request::{Request, SystemInstructionContent, SystemInstructionPart},
    Content, Part, Role,
};

use super::super::schemas::project_info::PROJECT_INFO;
use super::super::sysinst::project_info::PROJECT_INFO_SYSTEM_INSTRUCTION;

pub fn get_request(title: String) -> Request {
    Request {
        contents: get_contents(title),
        tools: vec![],
        safety_settings: vec![],
        generation_config: Some(PROJECT_INFO.deref().clone()),
        system_instruction: Some(SystemInstructionContent {
            parts: vec![SystemInstructionPart {
                text: Some(PROJECT_INFO_SYSTEM_INSTRUCTION.to_string()),
            }],
        }),
    }
}

fn get_contents(title: String) -> Vec<Content> {
    vec![Content {
        role: Role::User,
        parts: vec![Part {
            text: Some(format!("Project Title: {}", title)),
            inline_data: None,
            file_data: None,
            video_metadata: None,
        }],
    }]
}
