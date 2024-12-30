use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AlternativeTitles {
    #[serde(rename = "focusingOnAutomation")]
    pub automation: Vec<String>,
    #[serde(rename = "focusingOnInnovationOrProblemSolving")]
    pub innovation: Vec<String>,
    #[serde(rename = "focusingOnReports")]
    pub reports: Vec<String>,
    #[serde(rename = "moreConciseTitles")]
    pub concise: Vec<String>,
    #[serde(rename = "moreSpecificAndTechnicalTitles")]
    pub technical: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Titles {
    #[serde(rename = "originalTitle")]
    pub original: String,

    #[serde(rename = "recommendation")]
    pub recommendation: String,

    #[serde(rename = "alternativeTitles")]
    pub alternative_titles: AlternativeTitles,
}
