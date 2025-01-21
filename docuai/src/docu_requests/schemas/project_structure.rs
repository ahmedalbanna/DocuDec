use docubase::models::ReportMetadata;
use schema2struct::schema2struct;

schema2struct! {
    type: object,
    struct: ProjectStructure,
    required: ["structureAndFormat"],
    properties: {
        "structureAndFormat": {
            type: object,
            struct: key,
            required: ["targetLength", "chapterStructure", "visualAids"],
            properties: {
                "targetLength": { type: string },
                "chapterStructure": {
                    type: object,
                    struct: key,
                    required: ["description", "chapters"],
                    properties: {
                        "description": { type: string },
                        "chapters": { type: array, items: string }
                    }
                },
                "visualAids": {
                    type: object,
                    struct: key,
                    required: ["description"],
                    properties: { "description": { type: string } }
                }
            }
        }
    }
}

impl ReportMetadata for ProjectStructure {
    fn content(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn report_type(&self) -> docubase::models::ReportType {
        docubase::models::ReportType::ProjectStructure
    }
}

//pub(crate) fn create_project_structure_schema() -> Value {
//    let chapter_structure = Schema::new(SchemaType::Object)
//        .required(&["description", "chapters"])
//        .property("description", string!())
//        .property("chapters", array!(string!()));
//
//    let visual_aids = Schema::new(SchemaType::Object)
//        .required(&["description"])
//        .property("description", string!());
//
//    let structure_and_format = Schema::new(SchemaType::Object)
//        .required(&["targetLength", "chapterStructure", "visualAids"])
//        .property("targetLength", string!())
//        .property("chapterStructure", chapter_structure)
//        .property("visualAids", visual_aids);
//
//    Schema::new(SchemaType::Object)
//        .required(&["structureAndFormat"])
//        .property("structureAndFormat", structure_and_format)
//        .build()
//}
