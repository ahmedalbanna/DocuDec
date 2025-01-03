use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectContentRequiredSchema {
    pub description: String,
    pub steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectContentSchema {
    #[serde(alias = "problem_statement")]
    pub problem_statement: ProjectContentRequiredSchema,

    #[serde(alias = "research_questions")]
    pub research_questions: ProjectContentRequiredSchema,

    #[serde(alias = "literature_review")]
    pub literature_review: ProjectContentRequiredSchema,

    pub methodology: ProjectContentRequiredSchema,

    #[serde(alias = "system_design")]
    pub system_design: ProjectContentRequiredSchema,

    pub implementation: ProjectContentRequiredSchema,

    #[serde(alias = "testing_and_validation")]
    pub testing_and_validation: ProjectContentRequiredSchema,

    #[serde(alias = "results_and_analysis")]
    pub results_and_analysis: ProjectContentRequiredSchema,

    #[serde(alias = "future_work")]
    pub future_work: ProjectContentRequiredSchema,

    pub contributions: ProjectContentRequiredSchema,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectContent {
    #[serde(alias = "project_content")]
    pub project_content: ProjectContentSchema,
}
