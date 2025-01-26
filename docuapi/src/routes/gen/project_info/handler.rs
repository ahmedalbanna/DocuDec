use std::str::FromStr;
use std::sync::Arc;

use actix_web::web::{self, Json};
use actix_web::{HttpRequest, HttpResponse};
use docuai::docu_requests::schemas::ProjectInfo;
use docuai::{DocuError, DocuRequest};
use docubase::DocuBase;
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, error, instrument, trace};
use uuid::Uuid;

use crate::routes::gen::error::map_ai_error;

use super::super::error::GenError;
use super::request::ProjectInfoRequest;

use super::super::validators::validate_title::validate_title_request;

/// # SUCCESS
/// ```rust
/// {
///     // save this somewhere
///     "project_id": "6efac080-55e7-495f-a6a6-e9641440034e",
///     "content": { // project_info }
/// }
///
/// # Errors
/// ```
///
/// ```rust
/// GenError::JwtError => (400, "jwt error") // should not happen, but just in case,
/// GenError::UserDoesNotExists => (404, "the user doesn't exists") // should also not happen, but just in case,
/// GenError::ProjectExists => (409,"project alredy exists") // should happen when user typed the same title
/// GenError::ReportExists => (409, "report alredy exists") // should not happen in here
/// GenError::
///
/// ```
#[instrument(skip(model, db), fields(title = payload.title))]
#[actix_web::post("/project-info")]
pub async fn generate_project_info(
    model: web::Data<Arc<Client>>,
    db: web::Data<Arc<DocuBase>>,
    req: HttpRequest,
    payload: Json<ProjectInfoRequest>,
) -> Result<HttpResponse, GenError> {
    let jwt_token = req
        .cookie(crate::utils::constants::JWT_COOKIE)
        .expect("the endpoint is not accessable without the cookie, so we're fine");

    let user_id = docubase::utils::validate_jwt(jwt_token.value())
        // FIXME: it should properly not fail, but handle it just in case
        .map(|s| Uuid::from_str(&s).unwrap())?;

    let project_res = db.create_project(user_id, &payload.title).await?;

    validate_title_request(&payload)?;

    trace!(">> Generating project info for title: {}", payload.title);

    let result = generate_project_info_(model.as_ref(), &payload.title).await?;

    db.create_report(project_res.id, &result).await?;

    let response = serde_json::json!({
        "project_id": project_res.id,
        "content": result
    });

    trace!(">> Generated project info details: \n{:#?}", result);

    Ok(HttpResponse::Ok().json(response))
}

async fn generate_project_info_(model: &Client, title: &str) -> Result<ProjectInfo, GenError> {
    match docuai::request::<ProjectInfo>(model, DocuRequest::ProjectInfo(title.to_string()), 100)
        .await
    {
        Ok(project_info) => Ok(project_info),

        Err(error) => {
            error!("Project info generation failed: {:?}", error);

            if let DocuError::ApiRequest(ge) = error {
                error!(
                    "AI request failed. Error: {}, Status: {:?}",
                    ge.message, ge.code
                );

                Err(map_ai_error(&ge))
            } else {
                error!("Unrecoverable error in project info generation");

                Err(GenError::UnknownError)
            }
        }
    }
}
