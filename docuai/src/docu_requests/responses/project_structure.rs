use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ChapterStructure {
    pub description: String,
    pub chapters: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VisualAids {
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StructureAndFormat {
    #[serde(rename = "targetLength")]
    pub target_length: String,

    #[serde(rename = "chapterStructure")]
    pub chapter_structure: ChapterStructure,

    #[serde(rename = "visualAids")]
    pub visual_aids: VisualAids,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectStructure {
    #[serde(rename = "structureAndFormat")]
    pub structure_and_format: StructureAndFormat,
}
