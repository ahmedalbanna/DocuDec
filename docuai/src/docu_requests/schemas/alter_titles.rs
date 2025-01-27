use schema2struct::schema2struct;

use crate::DocuError;

schema2struct! {
    type: object,
    struct: AlternativeTitles,
    description: "Schema for analyzing and suggesting alternative titles for a project.",
    required: ["originalTitle", "recommendation", "alternativeTitles"],
    properties: {
        "originalTitle": { type: string },
        "recommendation": { type: string },
        "alternativeTitles": {
            type: object,
            struct: InnerAlternativeTitles,
            description: "Suggested alternative titles categorized by focus areas.",
            required: [
                "focusingOnAutomation",
                "focusingOnInnovationOrProblemSolving",
                "focusingOnReports",
                "moreConciseTitles",
                "moreSpecificAndTechnicalTitles",
            ],
            properties: {
                "focusingOnAutomation": { type: array, items: string },
                "focusingOnInnovationOrProblemSolving": { type: array, items: string },
                "focusingOnReports": { type: array, items: string },
                "moreConciseTitles": { type: array, items: string },
                "moreSpecificAndTechnicalTitles": { type: array, items: string }
            }

        }
    }
}

impl AlternativeTitles {
    #[must_use]
    pub fn to_json() -> serde_json::Value {
        (*ALTERNATIVETITLES_JSON_VALUE).clone()
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
        Ok(format!("alternativeTitles: {string:#?}\n"))
    }
}
