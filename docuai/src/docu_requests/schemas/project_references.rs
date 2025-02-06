use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::{
    docu_requests::{sysinsts, DocuComponent, RequestSchema, ToMessage},
    DocuError,
};

schema2struct! {
    type: object,
    struct: ProjectReferences,
    required: ["references"],
    properties: {
        "references": {
            type: array,
            items: {
                type: object,
                struct: ReferencesValue,
                required: ["authors", "title", "year", "publisher", "url"],
                properties: {
                    "authors": { type: string },
                    "title": { type: string },
                    "year": { type: string },
                    "publisher": { type: string },
                    "url": { type: string }
                }
            }
        }
    }

}

impl ReportMetadata for ProjectReferences {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectReferences
    }
}

impl ToMessage for ProjectReferences {
    fn write_message(&self, writer: &mut String) -> Result<(), DocuError> {
        writer.push_str(&format!(
            "projectReferences: {:#?}\n",
            serde_json::to_string(self)?
        ));

        Ok(())
    }
}

impl RequestSchema for ProjectReferences {
    fn to_json_value() -> serde_json::Value {
        PROJECTREFERENCES_JSON_VALUE.clone()
    }

    fn to_system_instruction<'a>(_is_plain: bool) -> &'a str {
        sysinsts::PROJECT_REFERENCES_SYSTEM_INSTRUCTION
    }
}

impl DocuComponent for ProjectReferences {}
