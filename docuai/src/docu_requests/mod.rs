//pub mod schema_builder;

pub mod requests;
//pub mod responses;
pub mod config;
pub mod context;
pub mod schemas;
pub mod sysinsts;

use context::{DocuRequest, ProcessRequest};

use google_generative_ai_rs::v1::gemini::request::{GenerationConfig, Request};

use serde_json::Value;

use crate::error::DocuError;

pub trait ToMessage {
    /// # Errors
    /// ```rust
    /// DocuError::JsonConversion(serde_json::Error)
    /// ```
    ///
    /// it returns an error if ``serde_json`` failed to convert the
    /// provided instance to json
    fn write_message(&self, writer: &mut String) -> Result<(), DocuError>;
}

pub trait RequestSchema {
    #[must_use]
    fn to_json_value() -> Value;

    #[must_use]
    fn to_system_instruction<'a>(is_plain: bool) -> &'a str;
}

pub trait DocuComponent: RequestSchema + ToMessage {}

/// # Errors
/// ```rust
/// DocuError::JsonConversion(serde_json::Error)
/// ```
///
/// it returns an error if ``serde_json`` failed to convert the
/// provided instance to json
pub fn to_request<R>(
    request: DocuRequest<'_, R>,
    config: GenerationConfig,
    is_plain: bool,
) -> Result<Request, DocuError>
where
    R: ProcessRequest,
{
    let mut message = String::new();
    let mut system_instruction = String::new();

    R::process_request(&request, &mut message, &mut system_instruction, is_plain)?;

    Ok(requests::get_request(message, config, system_instruction))
}
