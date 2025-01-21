use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

schema2struct! {
    type: object,
    struct: Abstract,
    required: ["r#abstract"],
    properties: {
        "r#abstract": { type: string }
    }

}

impl ReportMetadata for Abstract {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectAbstract
    }
}

//pub(crate) fn create_project_abstract_schema() -> Value {
//    Schema::new(SchemaType::Object)
//        .required(&["abstract"])
//        .property("abstract", string!())
//        .build()
//}
