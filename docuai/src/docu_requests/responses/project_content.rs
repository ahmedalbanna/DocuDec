use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectContentRequiredSchema {
    pub description: String,
    pub steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectContentSchema {
    #[serde(rename = "problemStatement")]
    pub problem_statement: ProjectContentRequiredSchema,

    #[serde(rename = "researchQuestions")]
    pub research_questions: ProjectContentRequiredSchema,

    #[serde(rename = "literatureReview")]
    pub literature_review: ProjectContentRequiredSchema,

    pub methodology: ProjectContentRequiredSchema,

    #[serde(rename = "systemDesign")]
    pub system_design: ProjectContentRequiredSchema,

    pub implementation: ProjectContentRequiredSchema,

    #[serde(rename = "testingAndValidation")]
    pub testing_and_validation: ProjectContentRequiredSchema,

    #[serde(rename = "resultsAndAnalysis")]
    pub results_and_analysis: ProjectContentRequiredSchema,

    #[serde(rename = "futureWork")]
    pub future_work: ProjectContentRequiredSchema,

    pub contributions: ProjectContentRequiredSchema,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectContent {
    #[serde(rename = "projectContent")]
    pub project_content: ProjectContentSchema,
}
