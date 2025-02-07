pub mod docu_requests;

mod error;

use docu_requests::{config, context::ProcessRequest, requests, RequestSchema};
pub use error::DocuError;

//pub use docu_requests::DocuRequest;
pub use docu_requests::context::DocuRequest;

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
    request: DocuRequest<'_, R>,
    timeout: u64,
) -> Result<R, DocuError>
where
    R: DeserializeOwned + ProcessRequest + RequestSchema,
{
    let request_object = crate::docu_requests::to_request(
        request,
        config::get_gen_config(R::to_json_value()),
        false,
    )?;
    let post_result = client.post(timeout, &request_object).await?;

    get_text(post_result)
        .map(|s| serde_json::from_str(&s).map_err(DocuError::JsonConversion))
        .ok_or(DocuError::NoTextFoundInResponse)?
}

/// # Errors
/// ```rust
///
/// // something wrong with the ai api
/// DocuError::ApiRequest(GoogleAPIError),
///
/// // there was no text in the ai response
/// DocuError::NoTextFoundInResponse
/// ```
pub async fn request_plain<R>(
    client: &Client,
    request: DocuRequest<'_, R>,
    timeout: u64,
) -> Result<String, DocuError>
where
    R: ProcessRequest + RequestSchema,
{
    let request_object =
        crate::docu_requests::to_request(request, config::get_plain_gen_config(), true)?;
    let post_result = client.post(timeout, &request_object).await?;

    get_text(post_result).ok_or(DocuError::NoTextFoundInResponse)
}

/// gets the text output of the gemini ai response
#[must_use]
pub fn get_text(req_result: PostResult) -> Option<String> {
    req_result
        .rest()?
        .candidates
        .into_iter()
        .find_map(|cand| cand.content.parts.into_iter().find_map(|p| p.text))
}

/// gets the gemini ai Model
///
/// it would return None if the env ``GOOGLE_API_KEY`` is not set
#[must_use]
pub fn get_model() -> Option<Client> {
    Some(Client::new_from_model(
        Model::Gemini2_0Flash,
        std::env::var("GOOGLE_API_KEY").ok()?,
    ))
}
