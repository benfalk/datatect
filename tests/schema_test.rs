use datatect::Schema;
use serde_json::json;

// Simple Examples

fn simple_schema() -> Schema {
    let data = json!({
        "type": "object",
        "properties": {
            "foo": {
                "type": "string"
            }
        }
    });

    Schema::new(data)
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
    let data = json!({
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
    });

    Schema::new(data)
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
    Schema::new(json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "foo": {
                    "type": "string"
                }
            }
        }
    }))
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
