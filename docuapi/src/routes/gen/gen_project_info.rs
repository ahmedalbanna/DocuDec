use std::sync::Arc;

use axum::{extract::State, Json};
use docuai::{
    docu_requests::{responses::project_info::ProjectInfo, DocuRequest},
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use serde::Deserialize;
use tracing::{debug, error, info, instrument};

use super::error::{map_ai_error, GenError};

#[derive(Debug, Deserialize)]
pub struct Title {
    title: String,
}

#[instrument(skip(model))]
pub async fn generate_project_info(
    State(model): State<Arc<Client>>,
    Json(payload): Json<Title>,
) -> Result<Json<ProjectInfo>, GenError> {
    if payload.title.is_empty() {
        error!("The title cannot be empty");
        return Err(GenError::EmptyTitle);
    }

    info!(
        ">> Generating project info based on the title: {}",
        payload.title
    );

    let result = docuai::request::<ProjectInfo>(
        model.as_ref(),
        DocuRequest::ProjectInfo(payload.title),
        100,
    )
    .await;

    let result = result
    .map_err(|e| {
        if let DocuError::ApiRequest(ge) = e {
                error!(
                    "Couldn't complete the AI request for generating the project info, error: {}, status code: {:?}",
                    ge.message, ge.code
                );
                map_ai_error(ge)
            }
        else {
                error!("Unknown Error: {}", e);
                GenError::UnknownError
            }
    })?;

    debug!("\n>> Generated titles: \n\n{:#?}\n", result);

    Ok(Json(result))
}
