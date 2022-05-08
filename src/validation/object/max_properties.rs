use crate::traits::Size;

/// Size validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/object.html#size>
pub trait ValidateObjectMaxProperties {
    fn check(&self, max_properties: usize) -> bool;
}

impl<T> ValidateObjectMaxProperties for T
where
    T: Size,
{
    fn check(&self, max_properties: usize) -> bool {
        max_properties >= self.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    #[test]
    fn test_validate_object_max_properties_hash_map_type() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(ValidateObjectMaxProperties::check(&map, 3));
    }

    #[test]
    fn test_validate_object_max_properties_btree_map_type() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(ValidateObjectMaxProperties::check(&map, 3));
    }

    #[test]
    fn test_validate_object_max_properties_json_map_type() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(ValidateObjectMaxProperties::check(map, 4));
        assert!(ValidateObjectMaxProperties::check(map, 3));
    }

    #[test]
    fn test_validate_object_max_properties_is_false() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(!ValidateObjectMaxProperties::check(map, 2));
    }
}
