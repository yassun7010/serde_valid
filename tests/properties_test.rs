use serde_valid::Validate;

use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[test]
fn properties_hash_map_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(properties(min_properties = 3, max_properties = 3))]
        val: HashMap<String, String>,
    }

    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    map.insert("key3".to_string(), "value3".to_string());

    let s = TestStruct { val: map };
    assert!(s.validate().is_ok());
}

#[test]
fn properties_btree_map_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(properties(min_properties = 3, max_properties = 3))]
        val: BTreeMap<String, String>,
    }

    let mut map = BTreeMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    map.insert("key3".to_string(), "value3".to_string());

    let s = TestStruct { val: map };
    assert!(s.validate().is_ok());
}

#[test]
fn properties_json_map_type_test() {
    #[derive(Debug, Deserialize, Validate)]
    struct TestStruct {
        #[validate(properties(min_properties = 3, max_properties = 3))]
        val: serde_json::Map<String, serde_json::Value>,
    }

    let s: TestStruct = serde_json::from_value(json!({
        "val": {
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        }
    }))
    .unwrap();
    assert!(s.validate().is_ok());
}

#[test]
fn properties_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(properties(min_properties = 3, max_properties = 3))]
        val: BTreeMap<String, String>,
    }

    let mut map = BTreeMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());

    let s = TestStruct { val: map };
    assert!(s.validate().is_err());
}

#[test]
fn properties_hash_map_type_err_message_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(properties(min_properties = 3, max_properties = 3))]
        val: HashMap<String, String>,
    }

    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());

    let s = TestStruct { val: map };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "properties size of {\"key1\": \"value1\"} must be in `3 <= size <= 3`, but `1`."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn properties_btree_map_type_err_message_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(properties(min_properties = 3, max_properties = 3))]
        val: BTreeMap<String, String>,
    }

    let mut map = BTreeMap::new();
    map.insert("key1".to_string(), "value1".to_string());

    let s = TestStruct { val: map };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "properties size of {\"key1\": \"value1\"} must be in `3 <= size <= 3`, but `1`."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn properties_json_map_type_err_message_test() {
    #[derive(Debug, Deserialize, Validate)]
    struct TestStruct {
        #[validate(properties(min_properties = 3, max_properties = 3))]
        val: serde_json::Map<String, serde_json::Value>,
    }

    let s: TestStruct = serde_json::from_value(json!({
        "val": {
            "key1": "value1",
        }
    }))
    .unwrap();

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "properties size of {\"key1\":\"value1\"} must be in `3 <= size <= 3`, but `1`."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn range_custom_err_message_test() {
    fn error_message(_params: &serde_valid::validation::error::PropertiesParams) -> String {
        "this is custom message.".to_string()
    }

    #[derive(Debug, Deserialize, Validate)]
    struct TestStruct {
        #[validate(properties(min_properties = 3, max_properties = 3, message_fn(error_message)))]
        val: serde_json::Map<String, serde_json::Value>,
    }

    let s: TestStruct = serde_json::from_value(json!({
        "val": {
            "key1": "value1",
        }
    }))
    .unwrap();

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "this is custom message."
            ]
        }))
        .unwrap()
    );
}
