use schema2struct::schema2struct;

use crate::{
    docu_requests::{context::ProcessRequest, sysinsts, DocuComponent, RequestSchema, ToMessage},
    DocuError, DocuRequest,
};

schema2struct! {
    type: object,
    struct: AlternativeTitles,
    description: "Schema for analyzing and suggesting alternative titles for a project.",
    required: ["originalTitle", "recommendation", "alternativeTitles"],
    properties: {
        "originalTitle": { type: string },
        "recommendation": { type: string },
        "alternativeTitles": {
            type: object,
            struct: InnerAlternativeTitles,
            description: "Suggested alternative titles categorized by focus areas.",
            required: [
                "focusingOnAutomation",
                "focusingOnInnovationOrProblemSolving",
                "focusingOnReports",
                "moreConciseTitles",
                "moreSpecificAndTechnicalTitles",
            ],
            properties: {
                "focusingOnAutomation": { type: array, items: string },
                "focusingOnInnovationOrProblemSolving": { type: array, items: string },
                "focusingOnReports": { type: array, items: string },
                "moreConciseTitles": { type: array, items: string },
                "moreSpecificAndTechnicalTitles": { type: array, items: string }
            }

        }
    }
}

//impl ToMessage for AlternativeTitles {
//    fn write_message(&self, writer: &mut String) -> Result<(), DocuError> {
//        writer.push_str(&format!(
//            "alternativeTitles: {:#?}\n",
//            serde_json::to_string(self)?
//        ));
//
//        Ok(())
//    }
//}

impl RequestSchema for AlternativeTitles {
    fn to_json_value() -> serde_json::Value {
        ALTERNATIVETITLES_JSON_VALUE.clone()
    }

    fn to_system_instruction<'a>(_is_plain: bool) -> &'a str {
        sysinsts::ALTER_TITLES_SYSTEM_INSTRUCTION
    }
}

//impl DocuComponent for AlternativeTitles {}

impl ProcessRequest for AlternativeTitles {
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
                titles: Some(title),
                ..
            } => {
                message.push_str(title);
                system_instruction.push_str(Self::to_system_instruction(is_plain));
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
