use std::{str::FromStr as _, sync::Arc};

use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use docuai::docu_requests::{schemas::ProjectContent, DocuRequest};
use docubase::{models::Report, DocuBase};
use google_generative_ai_rs::v1::api::Client;
use tokio::sync::Mutex;
use tracing::{instrument, trace};
use uuid::Uuid;

use crate::routes::gen::generate_report;

use super::super::error::GenError;
use super::request::ProjectContentRequest;

use super::super::payload_getters;
use once_cell::sync::Lazy;

// will be used to store the reports coming from the database, so it can be used more than once
static REPORTS: Lazy<Arc<Mutex<Option<Vec<Report>>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

/// # Request
/// ```json
/// {
///     "project_id": "...",
///
///     // optional if you have it, or it will be taken from the database if avaliable
///     "project_info": { ... }
/// }
/// ```
///
/// # Success
/// {
///     [`ProjectContent`](../../../../../docuai/docu_requests/schemas/project_content/struct.ProjectContent.html)
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
/// // if the user did not provide the project_info, and the api is trying to get it from the
/// // database instead, it will return a list of missing reports
/// GenError::MissingReports([...]) => (404, "missing reports: [ ... ]")
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
#[actix_web::post("/project-content")]
pub async fn generate_project_content(
    model: web::Data<Arc<Client>>,
    db: web::Data<Arc<DocuBase>>,
    req: HttpRequest,
    payload: Json<ProjectContentRequest>,
) -> Result<HttpResponse, GenError> {
    let jwt_token = req
        .cookie(crate::utils::constants::JWT_COOKIE)
        .expect("the endpoint is not accessable without the cookie, so we're fine");

    let user_id = docubase::utils::validate_jwt(jwt_token.value()).map(|s| {
        Uuid::from_str(&s).unwrap_or_else(|e| {
            panic!("can't convert the id to a Uuid, original is: {s}, error: {e}")
        })
    })?;

    // right here, we give it everything
    //
    // if the user provided the project_info, then we take that, nothing special, but if not, we
    // search through the user reports if he already done that, we take that, if not, then we
    // reutrn an error
    let project_info =
        payload_getters::project_info(&*payload, &payload.project_id, &db, &user_id, &REPORTS)
            .await?;

    trace!(
        ">> Generating project content based on the provided project info: {:#?}",
        project_info
    );

    let project_content = generate_report::<ProjectContent>(
        model.as_ref(),
        DocuRequest::ProjectContent(&project_info),
    )
    .await?;

    db.create_report(&payload.project_id, &user_id, &project_content)
        .await?;

    trace!(
        "\n>> Generated project content: \n\n{:#?}\n",
        project_content
    );

    Ok(HttpResponse::Ok().json(project_content))
}
