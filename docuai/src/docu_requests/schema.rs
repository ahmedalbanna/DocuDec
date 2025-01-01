use google_generative_ai_rs::v1::gemini::request::GenerationConfig;
use serde_json::Value;

pub(crate) fn get_gen_config(schema: Value) -> GenerationConfig {
    GenerationConfig {
        temperature: Some(1.0),
        top_p: Some(0.95),
        top_k: Some(40),
        candidate_count: None,
        max_output_tokens: Some(8192),
        stop_sequences: None,
        response_mime_type: Some("application/json".to_string()),
        response_schema: Some(schema),
    }
}
