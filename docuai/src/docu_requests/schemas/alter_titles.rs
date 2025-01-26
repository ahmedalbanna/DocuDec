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
    /// it returns an error if serde_json failed to convert the
    /// provided instance to json
    pub fn message(content: &Self) -> Result<String, DocuError> {
        let string = serde_json::to_string(content)?;
        Ok(format!("alternativeTitles: {string:#?}\n"))
    }
}

//pub fn create_alternative_titles_schema() -> Value {
//    let alter_titles = Schema::new(SchemaType::Object)
//        .description("Suggested alternative titles categorized by focus areas.")
//        .required(&[
//            "focusingOnAutomation",
//            "focusingOnInnovationOrProblemSolving",
//            "focusingOnReports",
//            "moreConciseTitles",
//            "moreSpecificAndTechnicalTitles",
//        ])
//        .property("focusingOnAutomation", array!(string!()))
//        .property("focusingOnInnovationOrProblemSolving", array!(string!()))
//        .property("focusingOnReports", array!(string!()))
//        .property("moreConciseTitles", array!(string!()))
//        .property("moreSpecificAndTechnicalTitles", array!(string!()));
//
//    Schema::new(SchemaType::Object)
//        .description("Schema for analyzing and suggesting alternative titles for a project.")
//        .required(&["originalTitle", "recommendation", "alternativeTitles"])
//        .property("originalTitle", string!())
//        .property("recommendation", string!())
//        .property("alternativeTitles", alter_titles)
//        .build()
//}
