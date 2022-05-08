use crate::traits::Size;

/// Size validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/object.html#size>
pub trait ValidateObjectMinProperties {
    fn check(&self, min_properties: usize) -> bool;
}

impl<T> ValidateObjectMinProperties for T
where
    T: Size,
{
    fn check(&self, min_properties: usize) -> bool {
        min_properties <= self.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    #[test]
    fn test_validate_object_min_properties_hash_map_type() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(ValidateObjectMinProperties::check(&map, 3));
    }

    #[test]
    fn test_validate_object_min_properties_btree_map_type() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(ValidateObjectMinProperties::check(&map, 3));
    }

    #[test]
    fn test_validate_object_min_properties_json_map_type() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(ValidateObjectMinProperties::check(map, 2));
        assert!(ValidateObjectMinProperties::check(map, 3));
    }

    #[test]
    fn test_validate_object_min_properties_is_false() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(!ValidateObjectMinProperties::check(map, 4));
    }
}
