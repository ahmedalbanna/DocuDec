#[test]
fn test_abstract_conversion_from_camelcase() {
    let json_data = crate::json_templates::project_abstract::camel_case::PROJECT_ABSTRACT_TEMPLATE;

    let r#abstract: docuai::docu_requests::schemas::ProjectAbstract =
        serde_json::from_str(json_data).expect("Failed to parse JSON to Abstract");

    assert!(!r#abstract.r#abstract.is_empty())
}

#[test]
fn test_abstract_conversion_from_snake_case() {
    let json_data = crate::json_templates::project_abstract::snake_case::PROJECT_ABSTRACT_TEMPLATE;

    let r#abstract: docuai::docu_requests::schemas::ProjectAbstract =
        serde_json::from_str(json_data).expect("Failed to parse JSON to Abstract");

    assert!(!r#abstract.r#abstract.is_empty())
}
