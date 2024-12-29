use crate::{
    array,
    docu_requests::schema_builder::{Schema, SchemaType},
    string,
};

use google_generative_ai_rs::v1::gemini::request::GenerationConfig;
use once_cell::sync::Lazy;
use serde_json::Value;

pub static ALTER_TITLES: Lazy<GenerationConfig> = Lazy::new(|| {
    GenerationConfig {
        temperature: Some(1.0),
        top_p: Some(0.95),
        top_k: Some(40),
        candidate_count: None,
        max_output_tokens: Some(8192),
        stop_sequences: None,
        response_mime_type: Some("application/json".to_string()),
        response_schema: Some(create_alternative_titles_schema()), // Use the created schema here
    }
});

fn create_alternative_titles_schema() -> Value {
    let alter_titles = Schema::new(SchemaType::Object)
        .description("Suggested alternative titles categorized by focus areas.")
        .required(&[
            "focusingOnAutomation",
            "focusingOnInnovationOrProblemSolving",
            "focusingOnReports",
            "moreConciseTitles",
            "moreSpecificAndTechnicalTitles",
        ])
        .property("focusingOnAutomation", array!(string!()))
        .property("focusingOnInnovationOrProblemSolving", array!(string!()))
        .property("focusingOnReports", array!(string!()))
        .property("moreConciseTitles", array!(string!()))
        .property("moreSpecificAndTechnicalTitles", array!(string!()));

    Schema::new(SchemaType::Object)
        .description("Schema for analyzing and suggesting alternative titles for a project.")
        .required(&["originalTitle", "recommendation", "alternativeTitles"])
        .property("originalTitle", string!())
        .property("recommendation", string!())
        .property("alternativeTitles", alter_titles)
        .build()
}
