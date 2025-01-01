use std::sync::Arc;

use axum::{
    extract::{rejection::JsonRejection, State},
    Json,
};
use docuai::{
    docu_requests::{
        responses::{
            project_content::ProjectContent, project_info::ProjectInfo,
            project_structure::ProjectStructure,
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
pub struct ProjectStructurePayload {
    project_info: ProjectInfo,
    project_content: ProjectContent,
}

#[instrument(skip(model))]
pub async fn generate_project_structure(
    State(model): State<Arc<Client>>,
    payload: Result<Json<ProjectStructurePayload>, JsonRejection>,
) -> Result<Json<ProjectStructure>, GenError> {
    let payload = match payload {
        Ok(p) => p.0,
        Err(err) => return Err(map_json_rejection::<ProjectStructurePayload>(err)),
    };

    info!(
        ">> Generating project content based on the provided project info: {:#?}\n\n project_content: {:#?}",
        payload.project_info, payload.project_content
    );

    let result = docuai::request::<ProjectStructure>(
        model.as_ref(),
        DocuRequest::ProjectStructure(&payload.project_info, &payload.project_content),
        100,
    )
    .await;

    let result = result
    .map_err(|e| {
        if let DocuError::ApiRequest(ge) = e {
                error!(
                    "Couldn't complete the AI request for generating the project structure, error: {}, status code: {:?}",
                    ge.message, ge.code
                );
                map_ai_error(ge)
            }
        else {
                error!("Unknown Error: {}", e);
                GenError::UnknownError
            }
    })?;

    debug!("\n>> Generated project structure: \n\n{:#?}\n", result);

    Ok(Json(result))
}
