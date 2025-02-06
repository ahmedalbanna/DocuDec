use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::{
    docu_requests::{sysinsts, DocuComponent, RequestSchema, ToMessage},
    DocuError,
};

schema2struct! {
    type: object,
    struct: ProjectStructure,
    required: ["structureAndFormat"],
    properties: {
        "structureAndFormat": {
            type: object,
            struct: key,
            required: ["targetLength", "chapterStructure", "visualAids"],
            properties: {
                "targetLength": { type: string },
                "chapterStructure": {
                    type: object,
                    struct: key,
                    required: ["description", "chapters"],
                    properties: {
                        "description": { type: string },
                        "chapters": { type: array, items: string }
                    }
                },
                "visualAids": {
                    type: object,
                    struct: key,
                    required: ["description"],
                    properties: { "description": { type: string } }
                }
            }
        }
    }
}

impl ReportMetadata for ProjectStructure {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectStructure
    }
}

impl ToMessage for ProjectStructure {
    fn write_message(&self, writer: &mut String) -> Result<(), DocuError> {
        writer.push_str(&format!(
            "projectStructure: {:#?}\n",
            serde_json::to_string(self)?
        ));
        Ok(())
    }
}

impl RequestSchema for ProjectStructure {
    fn to_json_value() -> serde_json::Value {
        PROJECTSTRUCTURE_JSON_VALUE.clone()
    }

    fn to_system_instruction<'a>(_is_plain: bool) -> &'a str {
        sysinsts::ALTER_TITLES_SYSTEM_INSTRUCTION
    }
}

impl DocuComponent for ProjectStructure {}
