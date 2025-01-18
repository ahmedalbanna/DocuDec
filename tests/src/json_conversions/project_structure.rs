#[test]
fn test_project_structure_conversion_from_camelcase() {
    let json_data =
        crate::json_templates::project_structure::camel_case::PROJECT_STRUCTURE_TEMPLATE;

    let project_structure: docuai::docu_requests::schemas::ProjectStructure =
        serde_json::from_str(json_data).expect("Failed to convert a JSON to a ProjectStructure");

    assert!(!project_structure
        .structure_and_format
        .visual_aids
        .description
        .is_empty())
}

#[test]
fn test_project_structure_conversion_from_snake_case() {
    let json_data =
        crate::json_templates::project_structure::snake_case::PROJECT_STRUCTURE_TEMPLATE;

    let project_structure: docuai::docu_requests::schemas::ProjectStructure =
        serde_json::from_str(json_data).expect("Failed to convert a JSON to a ProjectStructure");

    assert!(!project_structure
        .structure_and_format
        .visual_aids
        .description
        .is_empty())
}
