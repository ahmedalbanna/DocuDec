use std::sync::Arc;

use axum::{extract::State, Json};
use docuai::{
    docu_requests::{
        responses::{project_content::ProjectContent, project_info::ProjectInfo},
        DocuRequest,
    },
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use serde::Deserialize;
use tracing::{debug, error, info, instrument};

use super::error::{map_ai_error, GenError};

#[derive(Deserialize, Debug)]
pub struct ProjectContentPayload {
    pub project_info: ProjectInfo,
}

#[instrument(skip(model))]
pub async fn generate_project_content(
    State(model): State<Arc<Client>>,
    Json(payload): Json<ProjectContentPayload>,
) -> Result<Json<ProjectContent>, GenError> {
    info!(
        ">> Generating project content based on the provided project info: {:#?}",
        payload.project_info
    );

    let result = docuai::request::<ProjectContent>(
        model.as_ref(),
        DocuRequest::ProjectContent(&payload.project_info),
        100,
    )
    .await;

    let result = result
    .map_err(|e| {
        if let DocuError::ApiRequest(ge) = e {
                error!(
                    "Couldn't complete the AI request for generating the project content, error: {}, status code: {:?}",
                    ge.message, ge.code
                );
                map_ai_error(ge)
            }
        else {
                error!("Unknown Error: {}", e);
                GenError::UnknownError
            }
    })?;

    debug!("\n>> Generated project content: \n\n{:#?}\n", result);

    Ok(Json(result))
}
