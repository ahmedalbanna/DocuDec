use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::DocuError;

schema2struct! {
    type: object,
    struct: ProjectAbstract,
    required: ["r#abstract"],
    properties: {
        "r#abstract": { type: string }
    }

}

impl ReportMetadata for ProjectAbstract {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectAbstract
    }
}

impl ProjectAbstract {
    #[must_use]
    pub fn to_json() -> serde_json::Value {
        (*PROJECTABSTRACT_JSON_VALUE).clone()
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
        Ok(format!("abstract: {string:#?}\n"))
    }
}
