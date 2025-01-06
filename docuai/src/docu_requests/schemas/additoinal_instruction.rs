use crate::{
    docu_requests::schema_builder::{Schema, SchemaType},
    string,
};

use serde_json::Value;

pub(crate) fn create_additional_instruction_schema() -> Value {
    let additional_instructions = Schema::new(SchemaType::Object)
        .required(&["toneAndStyle", "keywords", "constraints"])
        .property("toneAndStyle", string!())
        .property("keywords", string!())
        .property("constraints", string!());

    Schema::new(SchemaType::Object)
        .required(&["additionalInstructions"])
        .property("additionalInstructions", additional_instructions)
        .build()
}
