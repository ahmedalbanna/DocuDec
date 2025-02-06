use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::{
    docu_requests::{sysinsts, DocuComponent, RequestSchema, ToMessage},
    DocuError,
};

schema2struct! {
    type: object,
    struct: ProjectAbstract,
    required: ["r#abstract"],
    properties: {
        "r#abstract": { type: string }
    }

}

impl ReportMetadata for ProjectAbstract {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectAbstract
    }
}

impl ToMessage for ProjectAbstract {
    fn write_message(&self, writer: &mut String) -> Result<(), DocuError> {
        writer.push_str(&format!("abstract: {:#?}\n", serde_json::to_string(self)?));

        Ok(())
    }
}

impl RequestSchema for ProjectAbstract {
    fn to_json_value() -> serde_json::Value {
        PROJECTABSTRACT_JSON_VALUE.clone()
    }

    fn to_system_instruction<'a>(_is_plain: bool) -> &'a str {
        sysinsts::PROJECT_ABSTRACT_SYSTEM_INSTRUCTION
    }
}

impl DocuComponent for ProjectAbstract {}
