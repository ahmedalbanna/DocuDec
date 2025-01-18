use jsonschema::jsonschema;

jsonschema! {
    type: object,
    struct: Abstract,
    required: ["r#abstract"],
    properties: {
        "r#abstract": { type: string }
    }

}

//pub(crate) fn create_project_abstract_schema() -> Value {
//    Schema::new(SchemaType::Object)
//        .required(&["abstract"])
//        .property("abstract", string!())
//        .build()
//}
