use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalInstructionsSchema {
    #[serde(rename = "toneAndStyle")]
    pub tone_and_style: String,
    pub keywords: String,
    pub constraints: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdditionalInstructions {
    #[serde(rename = "additionalInstructions")]
    pub additional_instructions: AdditionalInstructionsSchema,
}
