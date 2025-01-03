use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInstructionsSchema {
    #[serde(alias = "tone_and_style")]
    pub tone_and_style: String,
    pub keywords: String,
    pub constraints: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInstructions {
    #[serde(alias = "additional_instructions")]
    pub additional_instructions: AdditionalInstructionsSchema,
}
