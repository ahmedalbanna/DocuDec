use crate::{
    array,
    docu_requests::schema_builder::{Schema, SchemaType},
    string,
};

use serde_json::Value;

pub(crate) fn create_alternative_titles_schema() -> Value {
    let project_content_required_schema = Schema::new(SchemaType::Object)
        .required(&["description", "steps"])
        .property("description", string!())
        .property("steps", array!(string!()));

    let project_content = Schema::new(SchemaType::Object)
        .required(&[
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
        ])
        .property("problemStatement", project_content_required_schema.clone())
        .property("researchQuestions", project_content_required_schema.clone())
        .property("literatureReview", project_content_required_schema.clone())
        .property("methodology", project_content_required_schema.clone())
        .property("systemDesign", project_content_required_schema.clone())
        .property("implementation", project_content_required_schema.clone())
        .property(
            "testingAndValidation",
            project_content_required_schema.clone(),
        )
        .property(
            "resultsAndAnalysis",
            project_content_required_schema.clone(),
        )
        .property("futureWork", project_content_required_schema.clone())
        .property("contributions", project_content_required_schema.clone());

    Schema::new(SchemaType::Object)
        .required(&["projectContent"])
        .property("projectContent", project_content)
        .build()
}
