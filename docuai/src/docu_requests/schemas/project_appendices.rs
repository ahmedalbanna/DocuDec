use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::{
    docu_requests::{context::ProcessRequest, sysinsts, DocuComponent, RequestSchema, ToMessage},
    DocuError, DocuRequest,
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

impl ProcessRequest for ProjectAppendices {
    fn process_request(
        request: &crate::DocuRequest<Self>,
        message: &mut String,
        system_instruction: &mut String,
        is_plain: bool,
    ) -> Result<(), DocuError>
    where
        Self: Sized,
    {
        match request {
            DocuRequest {
                filled_info: Some(info),
                content: Some(content),
                structure: Some(structure),
                additional_instruction: Some(additional_instruction),
                abstrak: Some(abstrak),
                conclusion: Some(conclusion),
                references: Some(references),
                ..
            } => {
                info.write_message(message)?;
                content.write_message(message)?;
                structure.write_message(message)?;
                additional_instruction.write_message(message)?;
                abstrak.write_message(message)?;
                conclusion.write_message(message)?;
                references.write_message(message)?;

                system_instruction.push_str(Self::to_system_instruction(is_plain));
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
