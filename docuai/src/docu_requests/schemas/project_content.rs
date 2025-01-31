use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::DocuError;

schema2struct! {
    type: object,
    struct: ProjectContent,
    required: ["projectContent"],
    properties: {
        "projectContent": {
            type: object,
            struct: InnerProjectContent,
            required: [
                "problemStatement",
                "researchQuestions",
                "literatureReview",
                "methodology",
                "systemDesign",
                "implementation",
                "testingAndValidation",
                "resultsAndAnalysis",
                "futureWork",
                "contributions",
            ],
            properties: {
                "problemStatement": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "researchQuestions": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "literatureReview": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "methodology": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "systemDesign": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "implementation": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "testingAndValidation": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "resultsAndAnalysis": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "futureWork": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "contributions": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                }
            }
        }
    }
}

impl ReportMetadata for ProjectContent {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectContent
    }
}

impl ProjectContent {
    #[must_use]
    pub fn to_json() -> serde_json::Value {
        (*PROJECTCONTENT_JSON_VALUE).clone()
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
        Ok(format!("projectContent: {string:#?}\n"))
    }
}
