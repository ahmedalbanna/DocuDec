use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::DocuError;

schema2struct! {
    type: object,
    struct: ProjectReferences,
    required: ["references"],
    properties: {
        "references": {
            type: array,
            items: {
                type: object,
                struct: ReferencesValue,
                required: ["authors", "title", "year", "publisher", "url"],
                properties: {
                    "authors": { type: string },
                    "title": { type: string },
                    "year": { type: string },
                    "publisher": { type: string },
                    "url": { type: string }
                }
            }
        }
    }

}

impl ReportMetadata for ProjectReferences {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectReferences
    }
}

impl ProjectReferences {
    #[must_use]
    pub fn to_json() -> serde_json::Value {
        (*PROJECTREFERENCES_JSON_VALUE).clone()
    }

    /// # Errors
    /// ```rust
    /// DocuError::JsonConversion(serde_json::Error)
    /// ```
    ///
    /// it returns an error if serde_json failed to convert the
    /// provided instance to json
    pub fn message(content: &Self) -> Result<String, DocuError> {
        let string = serde_json::to_string(content)?;
        Ok(format!("projectReferences: {string:#?}\n"))
    }
}
