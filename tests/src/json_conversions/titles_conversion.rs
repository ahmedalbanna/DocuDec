#[test]
fn test_titles_conversion_from_camelcase() {
    let json_data = crate::json_templates::titles::camel_case::TITLES_TEMPLATE;

    let _: docuai::docu_requests::responses::alter_titles::Titles =
        serde_json::from_str(json_data).expect("Failed to parse the json to Titles");
}

#[test]
fn test_titles_conversion_from_snake_case() {
    let json_data = crate::json_templates::titles::snake_case::TITLES_TEMPLATE;

    let _: docuai::docu_requests::responses::alter_titles::Titles =
        serde_json::from_str(json_data).expect("Failed to parse the json to Titles");
}
