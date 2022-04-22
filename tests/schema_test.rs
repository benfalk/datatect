use datatect::Schema;
use serde_json::json;

// Simple Examples

fn simple_schema() -> Schema {
    json!({
        "type": "object",
        "properties": {
            "foo": {
                "type": "string"
            }
        }
    }).into()
}

#[test]
fn validating_the_happy_path() {
    let data = json!({"foo": "bar"});
    assert!(simple_schema().is_valid(&data));
}

#[test]
fn validating_wrong_type() {
    let data = json!({"foo": true});
    assert!(!simple_schema().is_valid(&data));
}

#[test]
fn extra_fields_fail_validation() {
    let data = json!({
        "foo": "bar",
        "rofl": "copter",
    });
    assert!(!simple_schema().is_valid(&data));
}

#[test]
fn missing_properties_fail_validation() {
    let data = json!({});
    assert!(!simple_schema().is_valid(&data));
}

// Nested Examples

fn nested_schema() -> Schema {
    json!({
        "type": "object",
        "properties": {
            "foo": {
                "type": "string"
            },
            "bar": {
                "type": "object",
                "properties": {
                    "biz": {
                        "type": "boolean"
                    }
                }
            }
        }
    }).into()
}

#[test]
fn passess_with_nested() {
    let data = json!({
        "foo": "bar",
        "bar": {
            "biz": true
        }
    });

    assert!(nested_schema().is_valid(&data));
}

#[test]
fn fails_with_nested_wrong_type() {
    let data = json!({
        "foo": "bar",
        "bar": {
            "biz": "baz"
        }
    });

    assert!(!nested_schema().is_valid(&data));
}

#[test]
fn nested_type_validation_errors() {
    let data = json!({
        "foo": "bar",
        "bar": {
            "biz": "baz"
        }
    });

    let schema = nested_schema();
    let result = schema.validate(&data);
    let errors: Vec<_> = result.unwrap_err().collect();

    assert_eq!(errors.len(), 1);

    assert_eq!(
        "\"baz\" is not of type \"boolean\"",
        errors[0].to_string()
    );

    assert_eq!(
        "/bar/biz",
        errors[0].instance_path.to_string()
    );
}

#[test]
fn fails_with_nested_extra_properties() {
    let data = json!({
        "foo": "bar",
        "bar": {
            "biz": true,
            "rofl": "copter"
        }
    });

    assert!(!nested_schema().is_valid(&data));
}

#[test]
fn fails_with_nested_missing_properties() {
    let data = json!({
        "foo": "bar",
        "bar": {
        }
    });

    assert!(!nested_schema().is_valid(&data));
}

// Testing with arrays

fn array_schema() -> Schema {
    json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "foo": {
                    "type": "string"
                }
            }
        }
    }).into()
}

#[test]
fn passes_array_of_objects() {
    let data = json!([{"foo":"bar"}]);
    assert!(array_schema().is_valid(&data));
}

#[test]
fn fails_an_object_in_array_with_additional_properties() {
    let data = json!([
        {"foo":"bar", "biz": false}
    ]);
    assert!(!array_schema().is_valid(&data));
}

#[test]
fn fails_an_object_in_array_with_missing_properties() {
    let data = json!([{}]);
    assert!(!array_schema().is_valid(&data));
}

#[test]
fn fails_an_object_in_array_with_incorrect_properties() {
    let data = json!([{"foo": false}]);
    assert!(!array_schema().is_valid(&data));
}

// Test oneOf

fn one_of_schema_example() -> Schema {
    json!({
        "oneOf": [
            { "type": "null" },
            {
                "type": "object",
                "properties": {
                    "foo": {
                        "type": "string"
                    }
                }
            }
        ]
    }).into()
}

#[test]
fn one_of_valid_with_non_object() {
    let data = json!(null);
    assert!(one_of_schema_example().is_valid(&data));
}

#[test]
fn one_of_valid_with_correct_object() {
    let data = json!({"foo": "bar"});
    assert!(one_of_schema_example().is_valid(&data));
}

#[test]
fn one_of_fails_with_object_with_incorrect_type() {
    let data = json!({"foo": false});
    assert!(!one_of_schema_example().is_valid(&data));
}

#[test]
fn one_of_fails_with_object_having_extra_properties() {
    let data = json!({"foo": "bar", "rofl": false});
    assert!(!one_of_schema_example().is_valid(&data));
}

#[test]
fn one_of_fails_with_object_missing_properties() {
    let data = json!({});
    assert!(!one_of_schema_example().is_valid(&data));
}
