use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::{
    docu_requests::{sysinsts, DocuComponent, RequestSchema, ToMessage},
    DocuError,
};

schema2struct! {
    type: object,
    struct: ProjectContent,
    required: ["projectContent"],
    properties: {
        "projectContent": {
            type: object,
            struct: InnerProjectContent,
            required: [
                "problemStatement",
                "researchQuestions",
                "literatureReview",
                "methodology",
                "systemDesign",
                "implementation",
                "testingAndValidation",
                "resultsAndAnalysis",
                "futureWork",
                "contributions",
            ],
            properties: {
                "problemStatement": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "researchQuestions": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "literatureReview": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "methodology": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "systemDesign": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "implementation": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "testingAndValidation": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "resultsAndAnalysis": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "futureWork": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                },
                "contributions": {
                    type: object,
                    struct: key,
                    required: ["description", "steps"],
                    properties: { "description": { type: string }, "steps": { type: array, items: string } }
                }
            }
        }
    }
}

impl ReportMetadata for ProjectContent {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectContent
    }
}

impl ToMessage for ProjectContent {
    fn write_message(&self, writer: &mut String) -> Result<(), DocuError> {
        writer.push_str(&format!(
            "projectContent: {:#?}\n",
            serde_json::to_string(self)?
        ));

        Ok(())
    }
}

impl RequestSchema for ProjectContent {
    fn to_json_value() -> serde_json::Value {
        PROJECTCONTENT_JSON_VALUE.clone()
    }

    fn to_system_instruction<'a>(_is_plain: bool) -> &'a str {
        sysinsts::PROJECT_CONTENT_SYSTEM_INSTRUCTION
    }
}

impl DocuComponent for ProjectContent {}
