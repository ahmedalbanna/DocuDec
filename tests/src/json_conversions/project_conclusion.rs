#[test]
fn test_project_conclusion_conversion_from_camelcase() {
    let json_data =
        crate::json_templates::project_conclusion::camel_case::PROJECT_CONCLUSION_TEMPLATE;

    let project_conclusion: docuai::docu_requests::schemas::ProjectConclusion =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectConclusion");

    assert!(!project_conclusion.conclusion.achievements.is_empty())
}

#[test]
fn test_project_conclusion_conversion_from_snake_case() {
    let json_data =
        crate::json_templates::project_conclusion::snake_case::PROJECT_CONCLUSION_TEMPLATE;

    let project_conclusion: docuai::docu_requests::schemas::ProjectConclusion =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectConclusion");

    assert!(!project_conclusion.conclusion.achievements.is_empty())
}
