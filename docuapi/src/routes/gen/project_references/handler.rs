use std::{str::FromStr as _, sync::Arc};

use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use docuai::docu_requests::{schemas::ProjectReferences, DocuRequest};
use docubase::{
    models::{Report, ReportType},
    DocuBase,
};
use google_generative_ai_rs::v1::api::Client;
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::{
    fetch_reports,
    routes::gen::{generate_report, update_action, UpdateAction},
};

use super::super::error::GenError;

use super::request::ProjectReferencesRequest;

/// # Request
/// ```json
/// {
///     "project_id": "...",
///
///     "update": bool,
///     "update_if_exist": bool,
///
///     // optional if you have it, or it will be taken from the database if avaliable
///     "project_info": { ... }
///     "project_content": { ... },
///     "project_structure": { .. },
///     "additional_instruction": { ... }
///     "project_abstract": { ... }
///     "project_conclusion": { ... }
/// }
/// ```
///
/// # Success
///
/// {
///     [`ProjectReferences`](../../../../../docuai/docu_requests/schemas/project_references/struct.ProjectReferences.html)
/// }
///
/// # Errors
/// ```rust
///
/// // if there is something wrong with the ai api
/// GenError::ApiRequestError => (500 , "Internal AI error: { String }"),
///
/// // if there is something wrong with the provided json
/// GenError::SyntaxError => (400, "Syntax error: { String }")
///
/// // if you set 'update' and 'update_if_exist' to true at the same time
/// GenError::UpdateMisuse => (400, "you can't use update and update_if_exist at the same time")
///
/// GenError::DBError => {
///
///        // internal database error
///        DocuBaseError::SqlxError => (500, "sqlx error: { sqlx::Error }")
///
///        // something wrong with validating the user jwt
///        DocuBaseError::JwtError(...) => (500, "jwt error: { jsonwebtoken::errors::Error }"),
///
///        // if for some reason the user have a valid jwt, but the user got deleted
///        DocuBaseError::UserDoesNotExist => (404, "user does not exist"),
///
///        // if the provided `project_id` does not exist
///        DocuBaseError::ProjectDoesNotExist => (404, "project does not exist"),
///
///        // if the user provided a `project_id` that does not belong to him
///        DocuBaseError::UserDoesNotOwnProject => (403, "user does not own the project"),
///
///        // this might happen if the user provided a report that he already did before
///        DocuBaseError::ReportExist => (409, "report (type) already exist"),
///
///        // this might happen if the user specifed to update the report, overriding the previous
///        // one, but there was no previous report
///        DocuBaseError::ReportDoesNotExist => (404, "report does not exist"),
///
///     }
/// ```
///
/// # Panics
///
/// it will panic if it couldn't get the cookie, which is impossible
///
/// or the user_id is not convertable to a Uuid
#[instrument(skip(model, db))]
#[actix_web::post("/project-references")]
pub async fn generate_project_references(
    model: web::Data<Arc<Client>>,
    db: web::Data<Arc<DocuBase>>,
    req: HttpRequest,
    payload: Json<ProjectReferencesRequest>,
) -> Result<HttpResponse, GenError> {
    let jwt_token = req
        .cookie(crate::utils::constants::JWT_COOKIE)
        .expect("the endpoint is not accessable without the cookie, so we're fine");

    let user_id = docubase::utils::validate_jwt(jwt_token.value()).map(|s| {
        Uuid::from_str(&s).unwrap_or_else(|e| {
            panic!("can't convert the id to a Uuid, original is: {s}, error: {e}")
        })
    })?;

    let update_action = update_action(payload.update, payload.update_if_exist)?;

    if matches!(update_action, UpdateAction::No) {
        db.check_report_type_exists(&payload.project_id, ReportType::ProjectReferences)
            .await?;
    }

    debug!(">> Generating project references");

    let mut reports: Option<Vec<Report>> = None;

    let (
        project_info,
        project_content,
        project_structure,
        additional_instruction,
        project_abstract,
        project_conclusion,
    ) = fetch_reports!(
        payload,
        db,
        user_id,
        reports,
        [
            ProjectInfo,
            ProjectContent,
            ProjectStructure,
            AdditionalInstruction,
            ProjectAbstract,
            ProjectConclusion
        ]
    );

    let request = DocuRequest::ProjectReferences(
        &project_info,
        &project_content,
        &project_structure,
        &additional_instruction,
        &project_abstract,
        &project_conclusion,
    );

    let project_references = generate_report::<ProjectReferences>(&model, request).await?;

    match update_action {
        UpdateAction::No => {
            db.create_report(&payload.project_id, &user_id, &project_references)
                .await?;
        }
        UpdateAction::Yes => {
            db.update_report(&payload.project_id, &user_id, &project_references)
                .await?;
        }
        UpdateAction::YesIfExist => {
            if db
                .report_type_exists(&payload.project_id, ReportType::ProjectReferences)
                .await?
            {
                db.delete_report_with_type(
                    ReportType::ProjectReferences,
                    &payload.project_id,
                    &user_id,
                )
                .await?;
            }
            db.create_report(&payload.project_id, &user_id, &project_references)
                .await?;
        }
    }

    debug!(
        "\n>> Generated project references: \n\n{:#?}\n",
        project_references
    );

    Ok(HttpResponse::Ok().json(project_references))
}
