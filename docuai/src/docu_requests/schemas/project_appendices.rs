use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::DocuError;

schema2struct! {
    type: object,
    struct: ProjectAppendices,
    required: ["appendices"],
    properties: {
        "appendices": {
            type: array,
            items: {
                type: object,
                struct: AppendicesValue,
                required: ["title", "description", "content"],
                properties: {
                    "title": { type: string },
                    "description": { type: string },
                    "content": { type: string }
                }
            }
        }
    }
}

impl ReportMetadata for ProjectAppendices {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectAppendices
    }
}

impl ProjectAppendices {
    #[must_use]
    pub fn to_json() -> serde_json::Value {
        (*PROJECTAPPENDICES_JSON_VALUE).clone()
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
        Ok(format!("projectAppendices: {string:#?}\n"))
    }
}
