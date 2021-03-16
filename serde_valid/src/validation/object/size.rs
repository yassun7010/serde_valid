use crate::traits::Size;

/// Size validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/object.html#id5>
pub fn validate_object_size<T>(
    value: &T,
    min_properties: Option<usize>,
    max_properties: Option<usize>,
) -> bool
where
    T: Size,
{
    let size = value.size();
    if let Some(max) = max_properties {
        if max < size {
            return false;
        }
    }

    if let Some(min) = min_properties {
        if size < min {
            return false;
        };
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    #[test]
    fn test_validate_object_size_hash_map_type() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(validate_object_size(&map, Some(3), Some(3)));
    }

    #[test]
    fn test_validate_object_size_btree_map_type() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(validate_object_size(&map, Some(3), Some(3)));
    }

    #[test]
    fn test_validate_object_size_json_map_type() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(validate_object_size(map, Some(3), Some(3)));
    }

    #[test]
    fn test_validate_object_size_is_false() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
        });
        let map = value.as_object().unwrap();

        assert!(!validate_object_size(map, Some(3), Some(3)));
    }
}
