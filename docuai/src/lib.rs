pub mod docu_requests;

mod error;
mod macros;

pub use error::DocuError;

pub use docu_requests::DocuRequest;

use google_generative_ai_rs::v1::{
    api::{Client, PostResult},
    gemini::Model,
};
use serde::de::DeserializeOwned;

/// # Errors
/// ```rust
///
/// // if `serde_json` failed
/// DocuError::JsonConversion(serde_json::Error)
///
/// // something wrong with the ai api
/// DocuError::ApiRequest(GoogleAPIError),
///
/// // there was no text in the ai response
/// DocuError::NoTextFoundInResponse
/// ```
pub async fn request<R>(
    client: &Client,
    request_type: DocuRequest<'_>,
    timeout: u64,
) -> Result<R, DocuError>
where
    R: DeserializeOwned,
{
    let request_object = request_type.to_request()?;

    let post_result = client.post(timeout, &request_object).await?;

    let response: R = request_type.to_response(post_result)?;

    Ok(response)
}

#[must_use]
pub fn get_text(req_result: PostResult) -> Option<String> {
    req_result
        .rest()?
        .candidates
        .into_iter()
        .find_map(|cand| cand.content.parts.last()?.text.clone())
}

#[must_use]
pub fn get_model() -> Option<Client> {
    Some(Client::new_from_model(
        Model::Gemini2_0Flash,
        std::env::var("GOOGLE_API_KEY").ok()?,
    ))
}
