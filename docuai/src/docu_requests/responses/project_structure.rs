use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ChapterStructure {
    pub description: String,
    pub chapters: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct VisualAids {
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StructureAndFormat {
    #[serde(alias = "target_length")]
    pub target_length: String,

    #[serde(alias = "chapter_structure")]
    pub chapter_structure: ChapterStructure,

    #[serde(alias = "visual_aids")]
    pub visual_aids: VisualAids,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStructure {
    #[serde(alias = "structure_and_format")]
    pub structure_and_format: StructureAndFormat,
}
