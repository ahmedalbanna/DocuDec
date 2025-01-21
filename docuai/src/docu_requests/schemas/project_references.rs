use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

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
        serde_json::to_value(self).unwrap()
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectReferences
    }
}
