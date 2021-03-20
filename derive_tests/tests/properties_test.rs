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
fn properties_err_message_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(properties(min_properties = 3, max_properties = 3))]
        val: HashMap<String, String>,
    }

    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());

    let s = TestStruct { val: map };

    let mut results = s.validate().unwrap_err().into_iter();
    let (field, errors) = results.next().unwrap();

    assert!(results.next().is_none());
    assert_eq!(field, "val");

    let mut errors = errors.iter();

    assert_eq!(
        format!("{}", errors.next().unwrap()),
        "properties size of {\"key1\": \"value1\"} must be in `3 <= size <= 3`, but `1`."
    );
    assert!(errors.next().is_none());
}
