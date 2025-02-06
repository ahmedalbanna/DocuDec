use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::{
    docu_requests::{sysinsts, DocuComponent, RequestSchema, ToMessage},
    DocuError,
};

schema2struct! {
    type: object,
    struct: AdditionalInstruction,
    required: ["additionalInstructions"],
    properties: {
        "additionalInstructions": {
            type: object,
            struct: InnerAdditionalInstructions,
            required: ["toneAndStyle", "keywords", "constraints"],
            properties: {
                "toneAndStyle": { type: string },
                "keywords": { type: string },
                "constraints": { type: string }
            }
        }
    }

}

impl ReportMetadata for AdditionalInstruction {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }
    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::AdditionalInstruction
    }
}

impl ToMessage for AdditionalInstruction {
    fn write_message(&self, writer: &mut String) -> Result<(), DocuError> {
        writer.push_str(&format!(
            "additionalInstructions: {:#?}\n",
            serde_json::to_string(self)?
        ));

        Ok(())
    }
}

impl RequestSchema for AdditionalInstruction {
    fn to_json_value() -> serde_json::Value {
        ADDITIONALINSTRUCTION_JSON_VALUE.clone()
    }

    fn to_system_instruction<'a>(_is_plain: bool) -> &'a str {
        sysinsts::ADDITIONAL_INSTRUCTION_SYSTEM_INSTRUCTION
    }
}

impl DocuComponent for AdditionalInstruction {}
