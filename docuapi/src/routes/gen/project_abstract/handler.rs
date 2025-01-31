use std::{str::FromStr as _, sync::Arc};

use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use docuai::docu_requests::{schemas::ProjectAbstract, DocuRequest};
use docubase::{models::Report, DocuBase};
use google_generative_ai_rs::v1::api::Client;
use tracing::{instrument, trace};
use uuid::Uuid;

use crate::{fetch_reports, routes::gen::generate_report};

use super::super::error::GenError;

use super::request::ProjectAbstractRequest;

/// # Request
/// ```json
/// {
///     "project_id": "...",
///
///     // optional if you have it, or it will be taken from the database if avaliable
///     "project_info": { ... }
///     "project_content": { ... },
///     "project_structure": { .. },
///     "additional_instruction": { ... }
/// }
/// ```
///
/// # Success
/// {
///     [`ProjectAbstract`](../../../../../docuai/docu_requests/schemas/project_abstract/struct.ProjectAbstract.html)
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
///        DocuBaseError::ProjectDoesNotExist => (404, "project does ot exist"),
///
///        // if the user provided a `project_id` that does not belong to him
///        DocuBaseError::UserDoesNotOwnProject => (403, "user does not own the project"),
///
///        // this might happen if the user provided a report that he already did before
///        DocuBaseError::ReportExist => (409, "report (type) alredy exist"),
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
#[actix_web::post("/project-abstract")]
pub async fn generate_project_abstract(
    model: web::Data<Arc<Client>>,
    db: web::Data<Arc<DocuBase>>,
    req: HttpRequest,
    payload: Json<ProjectAbstractRequest>,
) -> Result<HttpResponse, GenError> {
    let jwt_token = req
        .cookie(crate::utils::constants::JWT_COOKIE)
        .expect("the endpoint is not accessable without the cookie, so we're fine");

    let user_id = docubase::utils::validate_jwt(jwt_token.value()).map(|s| {
        Uuid::from_str(&s).unwrap_or_else(|e| {
            panic!("can't convert the id to a Uuid, original is: {s}, error: {e}")
        })
    })?;

    trace!(">> Generating project abstraction");

    let mut reports: Option<Vec<Report>> = None;

    let (project_info, project_content, project_structure, additional_instruction) = fetch_reports!(
        payload,
        db,
        user_id,
        reports,
        [
            ProjectInfo,
            ProjectContent,
            ProjectStructure,
            AdditionalInstruction
        ]
    );

    let request = DocuRequest::ProjectAbstract(
        &project_info,
        &project_content,
        &project_structure,
        &additional_instruction,
    );

    let project_abstract = generate_report::<ProjectAbstract>(model.as_ref(), request).await?;

    trace!(
        "\n>> Generated project abstraction: \n\n{:#?}\n",
        project_abstract
    );

    db.create_report(&payload.project_id, &user_id, &project_abstract)
        .await?;

    Ok(HttpResponse::Ok().json(project_abstract))
}
