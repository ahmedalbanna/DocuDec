#[test]
fn test_project_references_conversion_from_camelcase() {
    let json_data =
        crate::json_templates::project_references::camel_case::PROJECT_REFERENCES_TEMPLATE;

    let project_references: docuai::docu_requests::schemas::ProjectReferences =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectReferences");

    assert!(!project_references.references.is_empty())
}

#[test]
fn test_project_references_conversion_from_snake_case() {
    let json_data =
        crate::json_templates::project_references::snake_case::PROJECT_REFERENCES_TEMPLATE;

    let project_references: docuai::docu_requests::schemas::ProjectReferences =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectReferences");

    assert!(!project_references.references.is_empty())
}
