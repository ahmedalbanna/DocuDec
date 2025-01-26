use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

use crate::DocuError;

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
        serde_json::to_value(self).unwrap()
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectInfo
    }
}

impl ProjectInfo {
    #[must_use]
    pub fn to_json() -> serde_json::Value {
        (*PROJECTINFO_JSON_VALUE).clone()
    }

    /// # Errors
    /// ```rust
    /// DocuError::JsonConversion(serde_json::Error)
    /// ```
    ///
    /// it returns an error if serde_json failed to convert the
    /// provided instance to json
    pub fn message(content: &Self) -> Result<String, DocuError> {
        let string = serde_json::to_string(content)?;
        Ok(format!("projectInfo: {string:#?}\n"))
    }
}

//pub(crate) fn create_project_info_schema() -> Value {
//    // --- ProjectScope
//
//    let resource_alloc = Schema::new(SchemaType::Object)
//        .required(&["teamResponsibilities"])
//        .property("teamResponsibilities", array!(string!()));
//
//    let timeline = Schema::new(SchemaType::Object)
//        .required(&["start", "end", "milestones"])
//        .property("start", string!())
//        .property("end", string!())
//        .property("milestones", array!(string!()));
//
//    let project_scope = Schema::new(SchemaType::Object)
//        .required(&[
//            "description",
//            "objectivesAndGoals",
//            "boundaries",
//            "stakeholderIdentification",
//            "timeline",
//            "resourceAllocation",
//            "riskAssessment",
//            "approvalProcess",
//        ])
//        .property("description", string!())
//        .property("objectivesAndGoals", array!(string!()))
//        .property("boundaries", string!())
//        .property("stakeholderIdentification", array!(string!()))
//        .property("timeline", timeline)
//        .property("resourceAllocation", resource_alloc)
//        .property("riskAssessment", string!())
//        .property("approvalProcess", string!());
//
//    // --- ProjectScope
//
//    // -----
//    let filed_of_study = Schema::new(SchemaType::Object)
//        .required(&["academicField", "justification"])
//        .property("academicField", string!())
//        .property("justification", array!(string!()));
//    // ----
//    // --- targetAudience
//
//    let demographics = Schema::new(SchemaType::Object)
//        .required(&["age", "gender", "location"])
//        .property("age", string!())
//        .property("gender", string!())
//        .property("location", string!());
//
//    let target_aud = Schema::new(SchemaType::Object)
//        .required(&[
//            "marketResearch",
//            "demographics",
//            "psychographics",
//            "audienceFeedback",
//            "customerDataAnalysis",
//            "audiencePersonas",
//        ])
//        .property("marketResearch", string!())
//        .property("demographics", demographics)
//        .property("psychographics", array!(string!()))
//        .property("audienceFeedback", array!(string!()))
//        .property("customerDataAnalysis", array!(string!()))
//        .property("audiencePersonas", array!(string!()));
//
//    // --- targetAudience
//
//    Schema::new(SchemaType::Object)
//        .required(&[
//            "projectTitle",
//            "projectScope",
//            "fieldOfStudy",
//            "targetAudience",
//        ])
//        .property("projectTitle", string!())
//        .property("projectScope", project_scope)
//        .property("fieldOfStudy", filed_of_study)
//        .property("targetAudience", target_aud)
//        .build()
//}
