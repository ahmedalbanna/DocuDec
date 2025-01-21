use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

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
        serde_json::to_value(self).unwrap()
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectContent
    }
}

//pub(crate) fn create_alternative_titles_schema() -> Value {
//    let project_content_required_schema = Schema::new(SchemaType::Object)
//        .required(&["description", "steps"])
//        .property("description", string!())
//        .property("steps", array!(string!()));
//
//    let project_content = Schema::new(SchemaType::Object)
//        .required(&[
//            "problemStatement",
//            "researchQuestions",
//            "literatureReview",
//            "methodology",
//            "systemDesign",
//            "implementation",
//            "testingAndValidation",
//            "resultsAndAnalysis",
//            "futureWork",
//            "contributions",
//        ])
//        .property("problemStatement", project_content_required_schema.clone())
//        .property("researchQuestions", project_content_required_schema.clone())
//        .property("literatureReview", project_content_required_schema.clone())
//        .property("methodology", project_content_required_schema.clone())
//        .property("systemDesign", project_content_required_schema.clone())
//        .property("implementation", project_content_required_schema.clone())
//        .property(
//            "testingAndValidation",
//            project_content_required_schema.clone(),
//        )
//        .property(
//            "resultsAndAnalysis",
//            project_content_required_schema.clone(),
//        )
//        .property("futureWork", project_content_required_schema.clone())
//        .property("contributions", project_content_required_schema.clone());
//
//    Schema::new(SchemaType::Object)
//        .required(&["projectContent"])
//        .property("projectContent", project_content)
//        .build()
//}
