#[test]
fn test_project_abstract_conversion_from_camelcase() {
    let json_data = crate::json_templates::r#abstract::camel_case::ABSTRACT_TEMPLATE;

    let project_content: docuai::docu_requests::schemas::Abstract =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectContent");

    assert!(!project_content.r#abstract.is_empty())
}

#[test]
fn test_project_abstract_conversion_from_snake_case() {
    let json_data = crate::json_templates::r#abstract::snake_case::ABSTRACT_TEMPLATE;

    let project_content: docuai::docu_requests::schemas::Abstract =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectContent");

    assert!(!project_content.r#abstract.is_empty())
}
