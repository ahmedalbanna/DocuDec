use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AlternativeTitles {
    #[serde(alias = "focusing_on_automation")]
    pub focusing_on_automation: Vec<String>,

    #[serde(alias = "focusing_on_innovation_or_problem_solving")]
    pub focusing_on_innovation_or_problem_solving: Vec<String>,

    #[serde(alias = "focusing_on_reports")]
    pub focusing_on_reports: Vec<String>,

    #[serde(alias = "more_concise_titles")]
    pub more_concise_titles: Vec<String>,

    #[serde(alias = "more_specific_and_technical_titles")]
    pub more_specific_and_technical_titles: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Titles {
    #[serde(alias = "original_title")]
    pub original_title: String,
    pub recommendation: String,

    #[serde(alias = "alternative_titles")]
    pub alternative_titles: AlternativeTitles,
}
