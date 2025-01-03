use crate::{
    array,
    docu_requests::schema_builder::{Schema, SchemaType},
    string,
};

use serde_json::Value;

pub(crate) fn create_alternative_titles_schema() -> Value {
    Schema::new(SchemaType::Object).build()
}
