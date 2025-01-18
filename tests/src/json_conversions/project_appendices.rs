#[test]
fn test_project_appendices_conversion_from_camelcase() {
    let json_data =
        crate::json_templates::project_appendices::camel_case::PROJECT_APPENDICES_TEMPLATE;

    let project_appendices: docuai::docu_requests::schemas::ProjectAppendices =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectAppendices");

    assert!(!project_appendices.appendices.is_empty())
}

#[test]
fn test_project_appendices_conversion_from_snake_case() {
    let json_data =
        crate::json_templates::project_appendices::snake_case::PROJECT_APPENDICES_TEMPLATE;

    let project_appendices: docuai::docu_requests::schemas::ProjectAppendices =
        serde_json::from_str(json_data).expect("Failed to parse JSON to ProjectAppendices");

    assert!(!project_appendices.appendices.is_empty())
}
