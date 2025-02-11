use std::{io::Write, str::FromStr as _, sync::Arc};

use actix_web::{
    web::{self, Json},
    HttpRequest, HttpResponse,
};
use docuai::DocuRequest;
use docubase::{
    models::{Report, ReportType},
    DocuBase,
};

use google_generative_ai_rs::v1::api::Client;
use tracing::{instrument, trace};
use uuid::Uuid;

use crate::{
    fetch_reports,
    routes::gen::{generate_report, update_action, UpdateAction},
};

use super::super::error::GenError;
use super::request::ProjectContentRequest;

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
/// // if you set 'update' and 'update_if_exist' to true at the same time
/// GenError::UpdateMisuse => (400, "you can't use update and update_if_exist at the same time")
///
/// // if the user did not provide the necessary previous data, and the api is trying to get it from the
/// // database instead, but couldn't get it, it will return the missing report
/// GenError::MissingReports(ReportType) => (404, "missing reports: { ReportType }")
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

    let update_action = update_action(payload.update, payload.update_if_exist)?;

    if matches!(update_action, UpdateAction::No) {
        db.check_report_type_exists(&payload.project_id, ReportType::ProjectContent)
            .await?;
    }

    let mut reports: Option<Vec<Report>> = None;

    // right here, we give it everything
    //
    // if the user provided the project_info, then we take that, nothing special, but if not, we
    // search through the user reports if he already done that, we take that, if not, then we
    // reutrn an error

    let project_info = fetch_reports!(payload, db, user_id, reports, [ProjectInfo]);

    trace!(
        ">> Generating project content based on the provided project info: {:#?}",
        project_info
    );

    //let chapter_1 =
    //    docuai::request_plain(&model, &DocuRequest::new().to_chapter_1(&project_info), 100)
    //        .await
    //        .expect("failed to generate chapter 1");
    //
    //println!("{}", chapter_1);
    //
    //let chapter_2 =
    //    docuai::request_plain(&model, &DocuRequest::new().to_chapter_2(&project_info), 100)
    //        .await
    //        .expect("failed to generate chapter 2");
    //
    //println!("{}", chapter_2);
    //
    //let chapter_3 =
    //    docuai::request_plain(&model, &DocuRequest::new().to_chapter_3(&project_info), 100)
    //        .await
    //        .expect("failed to generate chapter 3");
    //
    //println!("{}", chapter_3);
    //
    //let chapter_4 =
    //    docuai::request_plain(&model, &DocuRequest::new().to_chapter_4(&project_info), 100)
    //        .await
    //        .expect("failed to generate chapter 4");
    //
    //println!("{}", chapter_4);
    //
    //let chapter_5 =
    //    docuai::request_plain(&model, &DocuRequest::new().to_chapter_5(&project_info), 100)
    //        .await
    //        .expect("failed to generate chapter 5");
    //
    //println!("{}", chapter_5);
    //
    //let chapter_6 =
    //    docuai::request_plain(&model, &DocuRequest::new().to_chapter_6(&project_info), 100)
    //        .await
    //        .expect("failed to generate chapter 6");
    //
    //println!("{}", chapter_6);
    //
    //let chapter_7 =
    //    docuai::request_plain(&model, &DocuRequest::new().to_chapter_7(&project_info), 100)
    //        .await
    //        .expect("failed to generate chapter 7");
    //
    //println!("{}", chapter_7);
    //
    //let chapter_8 =
    //    docuai::request_plain(&model, &DocuRequest::new().to_chapter_8(&project_info), 100)
    //        .await
    //        .expect("failed to generate chapter 8");
    //
    //println!("{}", chapter_8);
    //
    //let chapter_9 =
    //    docuai::request_plain(&model, &DocuRequest::new().to_chapter_9(&project_info), 100)
    //        .await
    //        .expect("failed to generate chapter 9");
    //
    //println!("{}", chapter_9);
    //
    //let chapter_10 = docuai::request_plain(
    //    &model,
    //    &DocuRequest::new().to_chapter_10(&project_info),
    //    100,
    //)
    //.await
    //.expect("failed to generate chapter 10");
    //
    //println!("{}", chapter_10);
    //
    //let chapter_11 = docuai::request_plain(
    //    &model,
    //    &DocuRequest::new().to_chapter_11(&project_info),
    //    100,
    //)
    //.await
    //.expect("failed to generate chapter 11");
    //
    //println!("{}", chapter_11);
    //
    //let chapter_12 = docuai::request_plain(
    //    &model,
    //    &DocuRequest::new().to_chapter_12(&project_info),
    //    100,
    //)
    //.await
    //.expect("failed to generate chapter 12");
    //
    //println!("{}", chapter_12);
    //
    //let final_output = chapter_1
    //    + &chapter_2
    //    + &chapter_3
    //    + &chapter_4
    //    + &chapter_5
    //    + &chapter_6
    //    + &chapter_7
    //    + &chapter_8
    //    + &chapter_9
    //    + &chapter_10
    //    + &chapter_11
    //    + &chapter_12;
    //
    //std::fs::File::create_new("output.txt")
    //    .unwrap()
    //    .write_fmt(format_args!("{final_output}"))
    //    .unwrap();
    //
    let project_content = generate_report(
        model.as_ref().as_ref(),
        &DocuRequest::new().with_filled_info(&project_info),
    )
    .await?;

    match update_action {
        UpdateAction::No => {
            db.create_report(&payload.project_id, &user_id, &project_content)
                .await?;
        }
        UpdateAction::Yes => {
            db.update_report(&payload.project_id, &user_id, &project_content)
                .await?;
        }
        UpdateAction::YesIfExist => {
            if db
                .report_type_exists(&payload.project_id, ReportType::ProjectContent)
                .await?
            {
                db.delete_report_with_type(
                    ReportType::ProjectContent,
                    &payload.project_id,
                    &user_id,
                )
                .await?;
            }
            db.create_report(&payload.project_id, &user_id, &project_content)
                .await?;
        }
    }

    trace!(
        "\n>> Generated project content: \n\n{:#?}\n",
        project_content
    );

    Ok(HttpResponse::Ok().json(project_content))
}
