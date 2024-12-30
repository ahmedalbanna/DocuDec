use std::ops::Deref;

use google_generative_ai_rs::v1::gemini::{
    request::{Request, SystemInstructionContent, SystemInstructionPart},
    Content, Part, Role,
};

use super::super::schemas::alter_titles::ALTER_TITLES;
use super::super::sysinsts::alter_titles::ALTER_TITLES_SYSTEM_INSTRUCTION;

pub fn get_request(original_title: String) -> Request {
    Request {
        contents: get_contents(original_title),
        tools: vec![],
        safety_settings: vec![],
        generation_config: Some(ALTER_TITLES.deref().clone()),
        system_instruction: Some(SystemInstructionContent {
            parts: vec![SystemInstructionPart {
                text: Some(ALTER_TITLES_SYSTEM_INSTRUCTION.to_string()),
            }],
        }),
    }
}

fn get_contents(original_title: String) -> Vec<Content> {
    vec![Content {
        role: Role::User,
        parts: vec![Part {
            text: Some(format!("Project Title: {}", original_title)),
            inline_data: None,
            file_data: None,
            video_metadata: None,
        }],
    }]
}
