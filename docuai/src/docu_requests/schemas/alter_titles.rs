use schema2struct::schema2struct;

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
