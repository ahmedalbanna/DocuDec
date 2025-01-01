use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectInfo {
    #[serde(rename = "fieldOfStudy")]
    pub field_of_study: FieldOfStudy,
    #[serde(rename = "projectScope")]
    pub scope: ProjectScope,
    #[serde(rename = "projectTitle")]
    pub title: String,
    #[serde(rename = "targetAudience")]
    pub audience: TargetAudience,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FieldOfStudy {
    #[serde(rename = "academicField")]
    pub academic_field: String,
    #[serde(rename = "justification")]
    pub justifications: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectScope {
    #[serde(rename = "approvalProcess")]
    pub approval_process: String,
    #[serde(rename = "boundaries")]
    pub boundaries: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "objectivesAndGoals")]
    pub objectives: Vec<String>,
    #[serde(rename = "resourceAllocation")]
    pub resource_allocation: ResourceAllocation,
    #[serde(rename = "riskAssessment")]
    pub risk_assessment: String,
    #[serde(rename = "stakeholderIdentification")]
    pub stakeholders: Vec<String>,
    #[serde(rename = "timeline")]
    pub timeline: Timeline,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ResourceAllocation {
    #[serde(rename = "teamResponsibilities")]
    pub team_responsibilities: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Timeline {
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "milestones")]
    pub milestones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TargetAudience {
    #[serde(rename = "audienceFeedback")]
    pub feedback: Vec<String>,
    #[serde(rename = "audiencePersonas")]
    pub personas: Vec<String>,
    #[serde(rename = "customerDataAnalysis")]
    pub data_analysis: Vec<String>,
    #[serde(rename = "demographics")]
    pub demographics: Demographics,
    #[serde(rename = "marketResearch")]
    pub market_research: String,
    #[serde(rename = "psychographics")]
    pub psychographics: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Demographics {
    #[serde(rename = "age")]
    pub age: String,
    #[serde(rename = "gender")]
    pub gender: String,
    #[serde(rename = "location")]
    pub location: String,
}
