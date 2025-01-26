use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::DocuError;

schema2struct! {
    type: object,
    struct: ProjectConclusion,
    properties: {
        "conclusion": {
            type: object,
            struct: key,
            required: [
                "summaryOfFindings",
                "achievements",
                "lessonsLearned",
                "futureDirections",
                "finalThoughts",
            ],
            properties: {
                "summaryOfFindings": { type: string },
                "achievements": { type: string },
                "lessonsLearned": { type: string },
                "futureDirections": { type: string },
                "finalThoughts": { type: string },
            }
        }
    }
}

impl ReportMetadata for ProjectConclusion {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectConclusion
    }
}

impl ProjectConclusion {
    #[must_use]
    pub fn to_json() -> serde_json::Value {
        (*PROJECTCONCLUSION_JSON_VALUE).clone()
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
        Ok(format!("projectConclusion: {string:#?}\n"))
    }
}
