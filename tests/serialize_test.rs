use serde_json::json;
use serde_valid::json::ToJson;

#[test]
fn to_json_string_is_ok() {
    assert!(json!({"val": 10}).to_json_string().is_ok())
}
