use crate::{
    array,
    docu_requests::schema_builder::{Schema, SchemaType},
    string,
};

use jsonschema::jsonschema;
use serde_json::Value;

jsonschema! {
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

pub(crate) fn create_project_structure_schema() -> Value {
    let chapter_structure = Schema::new(SchemaType::Object)
        .required(&["description", "chapters"])
        .property("description", string!())
        .property("chapters", array!(string!()));

    let visual_aids = Schema::new(SchemaType::Object)
        .required(&["description"])
        .property("description", string!());

    let structure_and_format = Schema::new(SchemaType::Object)
        .required(&["targetLength", "chapterStructure", "visualAids"])
        .property("targetLength", string!())
        .property("chapterStructure", chapter_structure)
        .property("visualAids", visual_aids);

    Schema::new(SchemaType::Object)
        .required(&["structureAndFormat"])
        .property("structureAndFormat", structure_and_format)
        .build()
}
