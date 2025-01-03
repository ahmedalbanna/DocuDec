use std::{borrow::Cow, collections::HashMap, fmt::Display};

use serde::Serialize;
use serde_json::Value;

pub enum SchemaType {
    String,
    Array,
    Object,
}

impl SchemaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SchemaType::String => "string",
            SchemaType::Array => "array",
            SchemaType::Object => "object",
        }
    }
}

impl ToString for SchemaType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(Default, Clone, Serialize)]
pub struct Schema {
    #[serde(rename = "type")]
    schema_type: Cow<'static, str>,
    description: Option<Cow<'static, str>>,
    items: Option<Box<Schema>>,

    #[serde(rename = "required")]
    required_fields: Option<Vec<String>>,
    properties: Option<HashMap<String, Box<Schema>>>,
}

impl Schema {
    // Creates a base schema of the given type
    pub fn new(schema_type: SchemaType) -> Self {
        Schema {
            schema_type: Cow::Borrowed(schema_type.as_str()),
            ..Default::default()
        }
    }

    // Sets a description for the schema
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(Cow::Owned(description.to_string()));
        self
    }

    // Sets the required fields for the schema
    pub fn required(mut self, fields: &[&str]) -> Self {
        self.required_fields = Some(fields.iter().map(|&f| f.to_string()).collect());
        self
    }

    // Adds a property to the schema, which can be another object or a basic type (string, array, etc.)
    pub fn property(mut self, name: &str, schema: Schema) -> Self {
        self.properties
            .get_or_insert_with(HashMap::new) // Initializes `properties` if it's `None`
            .insert(name.to_string(), Box::new(schema));
        self
    }

    // Sets the 'items' property for the schema
    pub fn items(mut self, schema: Schema) -> Self {
        self.items = Some(Box::new(schema));
        self
    }
    pub fn build(self) -> Value {
        serde_json::to_value(self).unwrap()
    }

    /*
    pub fn build(self) -> Value {
        let mut schema = indexmap::IndexMap::new();

        schema.insert("type".to_string(), Value::String(self.schema_type));

        if let Some(items_schema) = self.items {
            schema.insert("items".to_string(), items_schema.build());
        }

        if let Some(description) = self.description {
            schema.insert("description".to_string(), Value::String(description));
        }

        if let Some(required_fields) = self.required_fields {
            schema.insert(
                "required".to_string(),
                Value::Array(required_fields.into_iter().map(Value::String).collect()),
            );
        }

        if let Some(properties) = self.properties {
            let mut properties_map = serde_json::Map::new();
            for (name, schema) in properties {
                properties_map.insert(name.clone(), schema.build());
            }
            schema.insert("properties".to_string(), Value::Object(properties_map));
        }

        Value::Object(schema.into_iter().collect())
    }
    */
}
