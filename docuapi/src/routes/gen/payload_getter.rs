use std::borrow::Cow;

use docubase::{
    models::{Report, ReportType},
    DocuBase,
};
use serde::Deserialize;
use tracing::trace;
use uuid::Uuid;

use super::error::GenError;

#[macro_export]
macro_rules! fetch_reports {
    ($payload:expr, $db:expr, $user_id:expr, $reports:expr, [$($report_type:ident),*]) => {
        ($($crate::routes::gen::payload_getter::get_report(
            &*$payload,
            &$payload.project_id,
            &$db,
            &$user_id,
            &mut $reports,
            docubase::models::ReportType::$report_type,
        ).await?),*)
    };
}

pub trait ProvidesReport<T> {
    fn get_report(&self) -> Option<&T>;
}

/// # Errors
/// ```rust
/// GenError::MissingReports(ReportType) => (404, "missing reports: {ReportType}")
/// ```
///
/// # Panics
///
/// it would only panic if the value gotten from the database couldn't be converted to a the
/// specifed report type
pub async fn get_report<'a, G, T>(
    payload: &'a T,
    project_id: &Uuid,
    db: &DocuBase,
    user_id: &Uuid,
    chached_reports: &mut Option<Vec<Report>>,
    report_type: ReportType,
) -> Result<Cow<'a, G>, GenError>
where
    G: Clone + for<'de> Deserialize<'de> + 'a,
    T: ProvidesReport<G>,
{
    let gotten_report = if let Some(pi) = payload.get_report() {
        Cow::Borrowed(pi)
    } else {
        let reports = if let Some(ref stored_reports) = chached_reports {
            stored_reports
        } else {
            let db_reports = db.get_project_reports(project_id, user_id).await?;
            *chached_reports = Some(db_reports);

            chached_reports
                .as_ref()
                .expect("we just put the reports to it, so we can get it just fine")
        };

        trace!("{reports:#?}");

        let report = reports.iter().find(|&report| report.r#type == report_type);

        let report_content = report
            .ok_or_else(|| GenError::MissingReports(report_type))?
            .content
            .clone();

        Cow::Owned(serde_json::from_value(report_content).unwrap_or_else(|err| 
            panic!("couldn't convert the report coming from the database to a {report_type}, error: {err}")
        ))

        //match reports.as_slice() {
        //    [report @ Report {
        //        content: report_content,
        //        ..
        //    }] if report.r#type == report_type => Cow::Owned(
        //        serde_json::from_value(report_content.clone()).unwrap_or_else(|err| panic!(
        //            "couldn't convert the report coming from the database to a {report_type}, error: {err}"
        //        )),
        //    ),
        //
        //    _ => return Err(GenError::MissingReports(report_type)),
        //}
    };

    Ok(gotten_report)
}
