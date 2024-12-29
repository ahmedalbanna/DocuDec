use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use docuai::{
    docu_requests::{DocuRequest, DocuRequestInfo},
    get_text, request,
};
use google_generative_ai_rs::v1::api::Client;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Title {
    title: String,
}

pub async fn generate_titles(
    State(model): State<Arc<Client>>,
    Json(payload): Json<Title>,
) -> impl IntoResponse {
    println!(">> Generating titles based on: {}", payload.title);

    let result = request(
        model.as_ref(),
        DocuRequest::AlterTitles,
        DocuRequestInfo::title(payload.title),
        100,
    )
    .await;

    let text = get_text(result.unwrap()).await.unwrap();

    // Parse the JSON string into a serde_json::Value
    let parsed_json: Value = serde_json::from_str(&text).expect("Failed to parse JSON");

    println!(
        "\t>> Generated titles: {}",
        serde_json::to_string_pretty(&parsed_json).unwrap()
    );
}
