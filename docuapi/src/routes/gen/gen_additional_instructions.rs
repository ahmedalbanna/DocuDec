use std::sync::Arc;

use axum::{
    extract::{rejection::JsonRejection, State},
    Json,
};
use docuai::{
    docu_requests::{
        responses::{
            additional_instructions::AdditionalInstructions, project_content::ProjectContent,
            project_info::ProjectInfo, project_structure::ProjectStructure,
        },
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
pub struct AdditionalInstructionsPayload {
    project_info: ProjectInfo,
    project_content: ProjectContent,
    project_structure: ProjectStructure,
}

#[instrument(skip(model))]
pub async fn generate_additional_instructions(
    State(model): State<Arc<Client>>,
    payload: Result<Json<AdditionalInstructionsPayload>, JsonRejection>,
) -> Result<Json<AdditionalInstructions>, GenError> {
    let payload = match payload {
        Ok(p) => p.0,
        Err(err) => return Err(map_json_rejection::<AdditionalInstructionsPayload>(err)),
    };

    info!(
        ">> Generating project content based on the provided project info: {:#?}\n\nproject_content: {:#?}\n\nproject_structure: {:#?}",
        payload.project_info, payload.project_content, payload.project_structure
    );

    let result = docuai::request::<AdditionalInstructions>(
        model.as_ref(),
        DocuRequest::AdditionalInstructoins(
            &payload.project_info,
            &payload.project_content,
            &payload.project_structure,
        ),
        100,
    )
    .await;

    let result = result
    .map_err(|e| {
        if let DocuError::ApiRequest(ge) = e {
                error!(
                    "Couldn't complete the AI request for generating the additions instructions, error: {}, status code: {:?}",
                    ge.message, ge.code
                );
                map_ai_error(ge)
            }
        else {
                error!("Unknown Error: {}", e);
                GenError::UnknownError
            }
    })?;

    debug!(
        "\n>> Generated additional instructions: \n\n{:#?}\n",
        result
    );

    Ok(Json(result))
}
