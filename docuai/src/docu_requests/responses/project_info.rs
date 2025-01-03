use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    #[serde(alias = "field_of_study")]
    pub field_of_study: FieldOfStudy,

    #[serde(alias = "project_scope")]
    pub project_scope: ProjectScope,

    #[serde(alias = "project_title")]
    pub project_title: String,

    #[serde(alias = "target_audience")]
    pub target_audience: TargetAudience,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FieldOfStudy {
    #[serde(alias = "academic_field")]
    pub academic_field: String,
    pub justification: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectScope {
    #[serde(alias = "approval_process")]
    pub approval_process: String,
    pub boundaries: String,
    pub description: String,

    #[serde(alias = "objectives_and_goals")]
    pub objectives_and_goals: Vec<String>,

    #[serde(alias = "resource_allocation")]
    pub resource_allocation: ResourceAllocation,

    #[serde(alias = "risk_assessment")]
    pub risk_assessment: String,

    #[serde(alias = "stakeholder_identification")]
    pub stakeholder_identification: Vec<String>,
    pub timeline: Timeline,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceAllocation {
    #[serde(alias = "team_responsibilities")]
    pub team_responsibilities: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Timeline {
    pub start: String,
    pub end: String,
    pub milestones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TargetAudience {
    #[serde(alias = "audience_feedback")]
    pub audience_feedback: Vec<String>,

    #[serde(alias = "audience_personas")]
    pub audience_personas: Vec<String>,

    #[serde(alias = "customer_data_analysis")]
    pub customer_data_analysis: Vec<String>,
    pub demographics: Demographics,

    #[serde(alias = "market_research")]
    pub market_research: String,
    pub psychographics: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Demographics {
    pub age: String,
    pub gender: String,
    pub location: String,
}
