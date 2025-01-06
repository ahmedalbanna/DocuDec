#[test]
fn test_project_abstract_conversion_from_camelcase() {
    let json_data = crate::json_templates::project_abstract::camel_case::PROJECT_ABSTRACT_TEMPLATE;

    let project_content: docuai::docu_requests::responses::project_abstract::ProjectAbstract =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectContent");

    assert!(!project_content.r#abstract.is_empty())
}

#[test]
fn test_project_abstract_conversion_from_snake_case() {
    let json_data = crate::json_templates::project_abstract::snake_case::PROJECT_ABSTRACT_TEMPLATE;

    let project_content: docuai::docu_requests::responses::project_abstract::ProjectAbstract =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectContent");

    assert!(!project_content.r#abstract.is_empty())
}
