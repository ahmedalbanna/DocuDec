#[test]
fn test_project_content_conversion_from_camelcase() {
    let json_data = crate::json_templates::project_content::camel_case::PROJECT_CONTENT_TEMPLATE;

    let project_content: docuai::docu_requests::schemas::ProjectContent =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectContent");

    assert!(!project_content
        .project_content
        .future_work
        .description
        .is_empty())
}

#[test]
fn test_project_content_conversion_from_snake_case() {
    let json_data = crate::json_templates::project_content::snake_case::PROJECT_CONTENT_TEMPLATE;

    let project_content: docuai::docu_requests::schemas::ProjectContent =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectContent");

    assert!(!project_content
        .project_content
        .future_work
        .description
        .is_empty())
}
