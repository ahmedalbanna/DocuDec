#[test]
fn test_project_details_conversion_from_camelcase() {
    let json_data = crate::json_templates::project_details::camel_case::PROJECT_DETAILS_TEMPLATE;

    let project_details: docuai::docu_requests::schemas::ProjectDetails =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectDetails");

    assert!(!project_details.project_details.overview.is_empty())
}

#[test]
fn test_project_details_conversion_from_snake_case() {
    let json_data = crate::json_templates::project_details::snake_case::PROJECT_DETAILS_TEMPLATE;

    let project_details: docuai::docu_requests::schemas::ProjectDetails =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectDetails");

    assert!(!project_details.project_details.overview.is_empty());
}
