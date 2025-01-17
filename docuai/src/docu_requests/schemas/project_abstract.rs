use crate::{
    docu_requests::schema_builder::{Schema, SchemaType},
    string,
};
use jsonschema::jsonschema;

jsonschema! {
    type: object,
    struct: ProjectAbstract,
    required: ["r#abstract"],
    properties: {
        "r#abstract": { type: string }
    }

}

use serde_json::Value;

pub(crate) fn create_project_abstract_schema() -> Value {
    Schema::new(SchemaType::Object)
        .required(&["abstract"])
        .property("abstract", string!())
        .build()
}
