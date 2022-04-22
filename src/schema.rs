use jsonschema::{Draft, JSONSchema, ErrorIterator};
use serde_json::Value as Json;

#[derive(Debug)]
pub struct Schema {
    json_schema: JSONSchema,
}

impl <J: Into<Json>> From<J> for Schema {
    fn from(json: J) -> Self {
        Schema::new(json.into())
    }
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

    pub fn validate<'a>(&'a self, data: &'a Json) -> Result<(), ErrorIterator<'a>> {
        self.json_schema.validate(data)
    }
}

fn enforce_properties_defined(data: &mut Json) {
    let mut empty = serde_json::Map::new();
    let mut none = Json::Null;

    let obj = match data.as_object_mut() {
        None => return,
        Some(map) => map,
    };

    match obj.get("type") {
        Some(Json::String(t)) if t == "object" => {
            obj.insert("additionalProperties".to_owned(), Json::Bool(false));

            obj.insert(
                "required".to_owned(),
                Json::Array(
                    obj.get("properties")
                        .unwrap_or(&none)
                        .as_object()
                        .unwrap_or(&empty)
                        .keys()
                        .cloned()
                        .map(Json::String)
                        .collect(),
                ),
            );

            obj.get_mut("properties")
                .unwrap_or(&mut none)
                .as_object_mut()
                .unwrap_or(&mut empty)
                .values_mut()
                .for_each(enforce_properties_defined);
        }
        Some(Json::String(t)) if t == "array" => {
            if let Some(data) = obj.get_mut("items") {
                enforce_properties_defined(data);
            }
        }
        _ => (),
    }

    if let Some(Json::Array(types)) = obj.get_mut("oneOf") {
        types.iter_mut().for_each(enforce_properties_defined);
    }
}
