use google_generative_ai_rs::v1::errors::GoogleAPIError;

#[derive(Debug, thiserror::Error)]
pub enum DocuError {
    #[error("Internal API request error: {0}")]
    ApiRequest(#[from] GoogleAPIError),

    #[error("The post request did not have a text in it")]
    NoTextFoundInResponse,

    #[error("Error whlile converting to a json object, error: {0}")]
    JsonConversion(#[from] serde_json::Error),
}
