use google_generative_ai_rs::v1::gemini::{
    request::{GenerationConfig, Request, SystemInstructionContent, SystemInstructionPart},
    Content, Part, Role,
};

pub fn get_request(message: String, config: GenerationConfig, sysinst: String) -> Request {
    Request {
        contents: get_contents(message),
        tools: vec![],
        safety_settings: vec![],
        generation_config: Some(config),
        system_instruction: Some(SystemInstructionContent {
            parts: vec![SystemInstructionPart {
                text: Some(sysinst),
            }],
        }),
    }
}

fn get_contents(message: String) -> Vec<Content> {
    vec![Content {
        role: Role::User,
        parts: vec![Part {
            text: Some(message),
            inline_data: None,
            file_data: None,
            video_metadata: None,
        }],
    }]
}
