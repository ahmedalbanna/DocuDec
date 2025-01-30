use std::{borrow::Cow, sync::Arc};

use docuai::docu_requests::schemas::{
    HasProjectContent, HasProjectInfo, ProjectContent, ProjectInfo,
};
use docubase::{
    models::{Report, ReportType},
    DocuBase,
};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::error::GenError;

/// # Errors
/// ```rust
/// GenError::MissingReports(ReportType::ProjectInfo) => (404, "missing reports: ReportType::ProjectInfo")
/// ```
///
/// # Panics
///
/// it would only panic if the value gotten from the database couldn't be converted to a ``ProjectInfo``
pub async fn project_info<'a, T: HasProjectInfo>(
    payload: &'a T,
    project_id: &Uuid,
    db: &DocuBase,
    user_id: &Uuid,
    reports: &Arc<Mutex<Option<Vec<Report>>>>,
) -> Result<Cow<'a, ProjectInfo>, GenError> {
    let project_info = if let Some(pi) = payload.get_project_info() {
        Cow::Borrowed(pi)
    } else {
        let mut reports = reports.lock().await;

        let reports = {
            if let Some(ref initliazed_repots) = *reports {
                initliazed_repots
            } else {
                let db_reports = db.get_project_reports(project_id, user_id).await?;
                *reports = Some(db_reports);

                reports
                    .as_ref()
                    .expect("we just put the reports to it, so we can get it just fine")
            }
        };

        match reports.as_slice() {
            [Report {
                content: project_info,
                r#type: ReportType::ProjectInfo,
                ..
            }] => Cow::Owned(serde_json::from_value(project_info.clone()).expect(
                "couldn't convert the project_info coming from the database to a ProjectInfo",
            )),

            _ => return Err(GenError::MissingReports(ReportType::ProjectInfo)),
        }
    };

    Ok(project_info)
}

/// # Errors
/// ```rust
/// GenError::MissingReports(ReportType::ProjectContent) => (404, "missing reports: ReportType::ProjectContent")
/// ```
///
/// # Panics
///
/// it would only panic if the value gotten from the database couldn't be converted to a ``ProjectContent``
pub async fn project_content<'a, T: HasProjectContent>(
    payload: &'a T,
    project_id: &Uuid,
    db: &DocuBase,
    user_id: &Uuid,
    reports: &Arc<Mutex<Option<Vec<Report>>>>,
) -> Result<Cow<'a, ProjectContent>, GenError> {
    let project_info = if let Some(pi) = payload.get_project_content() {
        Cow::Borrowed(pi)
    } else {
        let mut reports = reports.lock().await;

        let reports = {
            if let Some(ref initliazed_repots) = *reports {
                initliazed_repots
            } else {
                let db_reports = db.get_project_reports(project_id, user_id).await?;
                *reports = Some(db_reports);

                reports
                    .as_ref()
                    .expect("we just put the reports to it, so we can get it just fine")
            }
        };

        match reports.as_slice() {
            [Report {
                content: project_content,
                r#type: ReportType::ProjectContent,
                ..
            }] => Cow::Owned(serde_json::from_value(project_content.clone()).expect(
                "couldn't convert the project_content coming from the database to a ProjectContent",
            )),

            _ => return Err(GenError::MissingReports(ReportType::ProjectContent)),
        }
    };

    Ok(project_info)
}
