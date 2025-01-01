use std::fmt::Display;

use serde_json::Value;

pub enum SchemaType {
    String,
    Array,
    Object,
}

impl Display for SchemaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaType::Object => f.write_str("object"),
            SchemaType::String => f.write_str("string"),
            SchemaType::Array => f.write_str("array"),
        }
    }
}

#[derive(Default, Clone)]
pub struct Schema {
    schema_type: String,
    description: Option<String>,
    items: Option<Box<Schema>>,
    required_fields: Option<Vec<String>>,
    properties: Option<Vec<(String, Box<Schema>)>>,
}

impl Schema {
    // Creates a base schema of the given type
    pub fn new(schema_type: SchemaType) -> Self {
        Schema {
            schema_type: schema_type.to_string(),
            ..Default::default()
        }
    }

    // Sets a description for the schema
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
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
            .get_or_insert_with(Vec::new) // Initializes `properties` if it's `None`
            .push((name.to_string(), Box::new(schema)));
        self
    }

    // Sets the 'items' property for the schema
    pub fn items(mut self, schema: Schema) -> Self {
        self.items = Some(Box::new(schema));
        self
    }

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
}
