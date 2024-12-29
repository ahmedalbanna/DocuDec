pub mod docu_requests;
mod macros;

use docu_requests::{DocuRequest, DocuRequestInfo};
use dotenv::dotenv;
use google_generative_ai_rs::v1::{
    api::{Client, PostResult},
    errors::GoogleAPIError,
    gemini::{request::Request, Content, Model, Part, Role},
};

pub async fn request(
    client: &Client,
    request_type: DocuRequest,
    info: DocuRequestInfo,
    timeout: u64,
) -> Result<PostResult, GoogleAPIError> {
    let request_object = request_type.to_request(info);
    client.post(timeout, &request_object).await
}

pub async fn get_text(req_result: PostResult) -> Option<String> {
    req_result
        .rest()?
        .candidates
        .into_iter()
        .filter_map(|cand| cand.content.parts.last()?.text.clone())
        .next()
}

pub fn get_model() -> Client {
    dotenv().ok();
    Client::new_from_model(
        Model::Gemini2_0Flash,
        std::env::var("GOOGLE_API_KEY").unwrap().to_string(),
    )
}

#[tokio::test]
async fn testing_schema_gen() {}

#[allow(dead_code)]
async fn test() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new_from_model(
        Model::Gemini2_0Flash,
        std::env::var("GOOGLE_API_KEY").unwrap().to_string(),
    );

    let txt_request = Request {
        contents: vec![
            Content {
                role: Role::User,
                parts: vec![Part {
                    text: Some("my name is Abdullah".to_string()),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            },
            Content {
                role: Role::Model,
                parts: vec![Part {
                    text: Some("Ok".to_string()),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            },
            Content {
                role: Role::User,
                parts: vec![Part {
                    text: Some("What's my name".to_string()),
                    inline_data: None,
                    file_data: None,
                    video_metadata: None,
                }],
            },
        ],
        tools: vec![],
        safety_settings: vec![],
        generation_config: None,
        system_instruction: None,
    };

    let response = client.post(30, &txt_request).await?;

    println!("{:#?}", response);
    Ok(())
}
