use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::{
    docu_requests::{context::ProcessRequest, sysinsts, DocuComponent, RequestSchema, ToMessage},
    DocuError, DocuRequest,
};

schema2struct! {
    type: object,
    struct: ProjectConclusion,
    properties: {
        "conclusion": {
            type: object,
            struct: key,
            required: [
                "summaryOfFindings",
                "achievements",
                "lessonsLearned",
                "futureDirections",
                "finalThoughts",
            ],
            properties: {
                "summaryOfFindings": { type: string },
                "achievements": { type: string },
                "lessonsLearned": { type: string },
                "futureDirections": { type: string },
                "finalThoughts": { type: string },
            }
        }
    }
}

impl ProjectConclusion {}

impl ReportMetadata for ProjectConclusion {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectConclusion
    }
}

impl ToMessage for ProjectConclusion {
    fn write_message(&self, writer: &mut String) -> Result<(), DocuError> {
        writer.push_str(&format!(
            "projectConclusion: {:#?}\n",
            serde_json::to_string(self)?
        ));

        Ok(())
    }
}

impl RequestSchema for ProjectConclusion {
    fn to_json_value() -> serde_json::Value {
        PROJECTCONCLUSION_JSON_VALUE.clone()
    }

    fn to_system_instruction<'a>(_is_plain: bool) -> &'a str {
        sysinsts::PROJECT_CONCLUSION_SYSTEM_INSTRUCTION
    }
}

impl DocuComponent for ProjectConclusion {}

impl ProcessRequest for ProjectConclusion {
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
                ..
            } => {
                info.write_message(message)?;
                content.write_message(message)?;
                structure.write_message(message)?;
                additional_instruction.write_message(message)?;
                abstrak.write_message(message)?;

                system_instruction.push_str(Self::to_system_instruction(is_plain));
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
