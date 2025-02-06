use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::{
    docu_requests::{sysinsts, DocuComponent, RequestSchema, ToMessage},
    DocuError,
};

schema2struct! {
    type: object,
    struct: ProjectAppendices,
    required: ["appendices"],
    properties: {
        "appendices": {
            type: array,
            items: {
                type: object,
                struct: AppendicesValue,
                required: ["title", "description", "content"],
                properties: {
                    "title": { type: string },
                    "description": { type: string },
                    "content": { type: string }
                }
            }
        }
    }
}

impl ReportMetadata for ProjectAppendices {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectAppendices
    }
}

impl ToMessage for ProjectAppendices {
    fn write_message(&self, writer: &mut String) -> Result<(), DocuError> {
        writer.push_str(&format!(
            "projectAppendices: {:#?}",
            serde_json::to_string(self)?
        ));

        Ok(())
    }
}

impl RequestSchema for ProjectAppendices {
    fn to_json_value() -> serde_json::Value {
        PROJECTAPPENDICES_JSON_VALUE.clone()
    }

    fn to_system_instruction<'a>(_is_plain: bool) -> &'a str {
        sysinsts::PROJECT_APPENDICES_SYSTEM_INSTRUCTION
    }
}

impl DocuComponent for ProjectAppendices {}
