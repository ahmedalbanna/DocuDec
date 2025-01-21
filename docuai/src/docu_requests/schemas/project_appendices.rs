use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

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
        serde_json::to_value(self).unwrap()
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectAppendices
    }
}
