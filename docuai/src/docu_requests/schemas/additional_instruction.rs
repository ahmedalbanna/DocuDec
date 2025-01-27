use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::DocuError;

schema2struct! {
    type: object,
    struct: AdditionalInstruction,
    required: ["additionalInstructions"],
    properties: {
        "additionalInstructions": {
            type: object,
            struct: InnerAdditionalInstructions,
            required: ["toneAndStyle", "keywords", "constraints"],
            properties: {
                "toneAndStyle": { type: string },
                "keywords": { type: string },
                "constraints": { type: string }
            }
        }
    }

}

impl ReportMetadata for AdditionalInstruction {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }
    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::AdditionalInstruction
    }
}

impl AdditionalInstruction {
    #[must_use]
    pub fn to_json() -> serde_json::Value {
        (*ADDITIONALINSTRUCTION_JSON_VALUE).clone()
    }

    /// # Errors
    /// ```rust
    /// DocuError::JsonConversion(serde_json::Error)
    /// ```
    ///
    /// it returns an error if ``serde_json`` failed to convert the
    /// provided instance to json
    pub fn message(&self) -> Result<String, DocuError> {
        let string = serde_json::to_string(self)?;
        Ok(format!("additionalInstructions: {string:#?}\n"))
    }
}
