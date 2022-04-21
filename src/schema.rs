use jsonschema::{Draft, JSONSchema};
use serde_json::Value as Json;

#[derive(Debug)]
pub struct Schema {
    json_schema: JSONSchema,
}

impl Schema {
    pub fn new(mut data: Json) -> Self {
        enforce_properties_defined(&mut data);

        let json_schema = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .compile(&data)
            .expect("a valid schema");

        Self { json_schema }
    }

    pub fn is_valid(&self, data: &Json) -> bool {
        self.json_schema.is_valid(data)
    }
}

fn enforce_properties_defined(data: &mut Json) {
    let mut empty = serde_json::Map::new();

    if data["type"].as_str().unwrap_or_default() == "object" {
        data["additionalProperties"] = Json::Bool(false);

        data["required"] = Json::Array(
            data["properties"]
            .as_object()
            .unwrap_or(&empty)
            .keys()
            .cloned()
            .map(Json::String)
            .collect()
        );

        data["properties"]
            .as_object_mut()
            .unwrap_or(&mut empty)
            .values_mut()
            .for_each(enforce_properties_defined);
    }

    if data["type"].as_str().unwrap_or_default() == "array" {
        enforce_properties_defined(&mut data["items"]);
    }
}
