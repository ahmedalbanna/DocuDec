use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

use crate::error::DocuBaseError;

pub trait ReportMetadata {
    fn content(&self) -> Value;
    fn report_type(&self) -> ReportType;
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub project_id: Uuid,
    pub report_id: Uuid,
    pub r#type: ReportType,
    pub content: Value,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ReportType {
    ProjectInfo,
    ProjectContent,
    ProjectStructure,
    AdditionalInstruction,
    ProjectAbstract,
    ProjectReferences,
    ProjectConclusion,
    ProjectAppendices,
    Chapter1,
    Chapter2,
    Chapter3,
    Chapter4,
    Chapter5,
    Chapter6,
    Chapter7,
    Chapter8,
    Chapter9,
    Chapter10,
    Chapter11,
}

impl std::fmt::Display for ReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self)
                .unwrap_or_default()
                .trim_matches('"')
        )
    }
}

impl TryFrom<String> for ReportType {
    type Error = DocuBaseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "project_info" => Ok(Self::ProjectInfo),
            "project_content" => Ok(Self::ProjectContent),
            "project_structure" => Ok(Self::ProjectStructure),
            "additional_instruction" => Ok(Self::AdditionalInstruction),
            "project_abstract" => Ok(Self::ProjectAbstract),
            "project_references" => Ok(Self::ProjectReferences),
            "project_conclusion" => Ok(Self::ProjectConclusion),
            "project_appendices" => Ok(Self::ProjectAppendices),
            "chapter_1" => Ok(Self::Chapter1),
            "chapter_2" => Ok(Self::Chapter2),
            "chapter_3" => Ok(Self::Chapter3),
            "chapter_4" => Ok(Self::Chapter4),
            "chapter_5" => Ok(Self::Chapter5),
            "chapter_6" => Ok(Self::Chapter6),
            "chapter_7" => Ok(Self::Chapter7),
            "chapter_8" => Ok(Self::Chapter8),
            "chapter_9" => Ok(Self::Chapter9),
            "chapter_10" => Ok(Self::Chapter10),
            "chapter_11" => Ok(Self::Chapter11),
            _ => Err(DocuBaseError::InvalidReportType(value)),
        }
    }
}
