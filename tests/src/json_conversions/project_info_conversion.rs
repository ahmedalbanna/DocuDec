#[test]
fn test_project_info_conversion_from_camelcase() {
    let json_data = crate::json_templates::project_info::camel_case::PROJECT_INFO_TEMPLATE;

    let project_info: docuai::docu_requests::schemas::ProjectInfo =
        serde_json::from_str(json_data).expect("Failed to parse json to ProjectInfo");

    assert!(!project_info.field_of_study.academic_field.is_empty())
}

#[test]
fn test_project_info_conversion_from_snake_case() {
    let json_data = crate::json_templates::project_info::snake_case::PROJECT_INFO_TEMPLATE;

    let project_info: docuai::docu_requests::schemas::ProjectInfo =
        serde_json::from_str(json_data).expect("Failed to parse json to ProjectInfo");

    assert!(!project_info.field_of_study.academic_field.is_empty())
}
