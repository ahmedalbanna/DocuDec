use crate::{
    array,
    docu_requests::schema_builder::{Schema, SchemaType},
    string,
};

use google_generative_ai_rs::v1::gemini::request::GenerationConfig;
use once_cell::sync::Lazy;
use serde_json::Value;

pub static PROJECT_INFO: Lazy<GenerationConfig> = Lazy::new(|| {
    GenerationConfig {
        temperature: Some(1.0),
        top_p: Some(0.95),
        top_k: Some(40),
        candidate_count: None,
        max_output_tokens: Some(8192),
        stop_sequences: None,
        response_mime_type: Some("application/json".to_string()),
        response_schema: Some(create_project_info_schema()), // Use the created schema here
    }
});

fn create_project_info_schema() -> Value {
    // --- ProjectScope
    let resource_alloc = Schema::new(SchemaType::Object)
        .required(&["teamResponsibilities"])
        .property("teamResponsibilities", array!(string!()));

    let timeline = Schema::new(SchemaType::Object)
        .required(&["start", "end", "milestones"])
        .property("start", string!())
        .property("end", string!())
        .property("milestones", array!(string!()));

    let project_scope = Schema::new(SchemaType::Object)
        .required(&[
            "description",
            "objectivesAndGoals",
            "boundaries",
            "stakeholderIdentification",
            "timeline",
            "resourceAllocation",
            "riskAssessment",
            "approvalProcess",
        ])
        .property("description", string!())
        .property("objectivesAndGoals", array!(string!()))
        .property("boundaries", string!())
        .property("stakeholderIdentification", array!(string!()))
        .property("timeline", timeline)
        .property("resourceAllocation", resource_alloc)
        .property("riskAssessment", string!())
        .property("approvalProcess", string!());

    // --- ProjectScope

    // -----
    let filed_of_study = Schema::new(SchemaType::Object)
        .required(&["academicField", "justification"])
        .property("academicField", string!())
        .property("justification", array!(string!()));
    // ----

    // --- targetAudience

    let demographics = Schema::new(SchemaType::Object)
        .required(&["age", "gender", "location"])
        .property("age", string!())
        .property("gender", string!())
        .property("location", string!());

    let target_aud = Schema::new(SchemaType::Object)
        .required(&[
            "marketResearch",
            "demographics",
            "psychographics",
            "audienceFeedback",
            "customerDataAnalysis",
            "audiencePersonas",
        ])
        .property("marketResearch", string!())
        .property("demographics", demographics)
        .property("psychographics", array!(string!()))
        .property("audienceFeedback", array!(string!()))
        .property("customerDataAnalysis", array!(string!()))
        .property("audiencePersonas", array!(string!()));

    // --- targetAudience

    Schema::new(SchemaType::Object)
        .required(&[
            "projectTitle",
            "projectScope",
            "fieldOfStudy",
            "targetAudience",
        ])
        .property("projectTitle", string!())
        .property("projectScope", project_scope)
        .property("fieldOfStudy", filed_of_study)
        .property("targetAudience", target_aud)
        .build()
}
