use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

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

impl ReportMetadata for ProjectConclusion {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectConclusion
    }
}
