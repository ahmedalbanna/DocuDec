
#[test]
fn test_additional_instructions_conversion_from_camelcase() {
    let json_data = crate::json_templates::additional_instruction::camel_case::ADDITIONAL_INSTRUCTION_TEMPLATE;

    let additional_instructions: 
docuai::docu_requests::responses::additional_instruction::AdditionalInstructions = serde_json::from_str(json_data).expect("Failed to convert a JSON to AdditionalInstructions");

    assert!(!additional_instructions.additional_instructions.tone_and_style.is_empty())
}

#[test]
fn test_additional_instructions_conversion_from_snake_case() {
    let json_data = crate::json_templates::additional_instruction::snake_case::ADDITIONAL_INSTRUCTION_TEMPLATE;

    let additional_instructions: 
docuai::docu_requests::responses::additional_instruction::AdditionalInstructions = serde_json::from_str(json_data).expect("Failed to convert a JSON to AdditionalInstructions");

    assert!(!additional_instructions.additional_instructions.tone_and_style.is_empty())
}
