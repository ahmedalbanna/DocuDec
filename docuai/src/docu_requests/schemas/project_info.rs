use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::{
    docu_requests::{sysinsts, DocuComponent, RequestSchema, ToMessage},
    DocuError,
};

schema2struct! {
    type: object,
    struct: ProjectInfo,
    required: [
        "projectTitle",
        "projectScope",
        "fieldOfStudy",
        "targetAudience",
    ],
    properties: {
        "projectTitle": { type: string },
        "projectScope": {
            type: object,
            struct: key,
            required: [
                "description",
                "objectivesAndGoals",
                "boundaries",
                "stakeholderIdentification",
                "timeline",
                "resourceAllocation",
                "riskAssessment",
                "approvalProcess",
            ],
            properties: {
                "description": { type: string },
                "objectivesAndGoals": { type: array, items: string },
                "boundaries": { type: string },
                "stakeholderIdentification": { type: array, items: string },
                "riskAssessment": { type: string },
                "approvalProcess": { type: string },
                "timeline": {
                    type: object,
                    required: ["start", "end", "milestones"],
                    struct: key,
                    properties: {
                        "start": { type: string },
                        "end": { type: string },
                        "milestones": { type: array, items: string }
                    }
                },
                "resourceAllocation": {
                    type: object,
                    struct: key,
                    required: ["teamResponsibilities"],
                    properties: {
                        "teamResponsibilities": { type: array, items: string }
                    }
                }
            }
        },
        "fieldOfStudy": {
            type: object,
            struct: key,
            required: ["academicField", "justification"],
            properties: {
                "academicField": { type: string },
                "justification": { type: array, items: string }
            }
        },
        "targetAudience": {
            type: object,
            struct: key,
            required: [
                "marketResearch",
                "demographics",
                "psychographics",
                "audienceFeedback",
                "customerDataAnalysis",
                "audiencePersonas",
            ],
            properties: {
                "marketResearch": { type: string },
                "demographics": {
                    type: object,
                    struct: key,
                    required: ["age", "gender", "location"],
                    properties: {
                        "age": { type: string },
                        "gender": { type: string },
                        "location": { type: string }
                    }
                },
                "psychographics": { type: array, items: string },
                "audienceFeedback": { type: array, items: string },
                "customerDataAnalysis": { type: array, items: string },
                "audiencePersonas": { type: array, items: string }

            }
        },
    }
}

impl ReportMetadata for ProjectInfo {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|err| {
            panic!("couldn't serialize AdditionalInstruction into json, error: {err}",)
        })
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectInfo
    }
}

impl ToMessage for ProjectInfo {
    fn write_message(&self, writer: &mut String) -> Result<(), DocuError> {
        writer.push_str(&format!(
            "projectInfo: {:#?}\n",
            serde_json::to_string(self)?
        ));

        Ok(())
    }
}

impl RequestSchema for ProjectInfo {
    fn to_json_value() -> serde_json::Value {
        PROJECTINFO_JSON_VALUE.clone()
    }
    fn to_system_instruction<'a>(_is_plain: bool) -> &'a str {
        sysinsts::PROJECT_INFO_SYSTEM_INSTRUCTION
    }
}

impl DocuComponent for ProjectInfo {}
