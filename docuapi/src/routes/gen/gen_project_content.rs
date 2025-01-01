use std::sync::Arc;

use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    Json,
};
use docuai::{
    docu_requests::{
        responses::{project_content::ProjectContent, project_info::ProjectInfo},
        DocuRequest,
    },
    DocuError,
};
use google_generative_ai_rs::v1::api::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument};

use crate::routes::gen::error::map_json_rejection;

use super::error::{map_ai_error, GenError};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ProjectContentPayload {
    pub project_info: ProjectInfo,
}

#[instrument(skip(model))]
pub async fn generate_project_content(
    State(model): State<Arc<Client>>,
    payload: Result<Json<ProjectContentPayload>, JsonRejection>,
) -> Result<(StatusCode, Json<ProjectContent>), GenError> {
    let payload = match payload {
        Ok(p) => p.0,
        Err(err) => return Err(map_json_rejection::<ProjectContentPayload>(err)),
    };

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

    Ok((StatusCode::OK, Json(result)))
}
