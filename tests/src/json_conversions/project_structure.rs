#[test]
fn test_project_structure_conversion() {
    let json_data = r#"
{
  "structureAndFormat": {
    "chapterStructure": {
      "chapters": [
        "Introduction",
        "Literature Review",
        "Methodology",
        "Results",
        "Discussion",
        "Conclusion",
        "References",
        "Appendices"
      ],
      "description": "The report will be structured with standard academic sections, including an introduction, a review of relevant literature, a description of the methodology, a presentation and analysis of results, a discussion of implications, a conclusion, references, and appendices."
    },
    "targetLength": "10-15 pages",
    "visualAids": {
       "description": "The report will include diagrams generated from PlantUML, charts illustrating test results, and tables presenting data analysis, all with clear captions and labels."
    }
  }
}
    "#;

    let project_structure: docuai::docu_requests::responses::project_structure::ProjectStructure =
        serde_json::from_str(json_data).expect("Failed to convert a JSON to a ProjectStructure");

    assert!(!project_structure
        .structure_and_format
        .visual_aids
        .description
        .is_empty())
}
