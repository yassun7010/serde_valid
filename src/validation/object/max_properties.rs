use crate::{traits::Size, MaxPropertiesErrorParams};

/// Size validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/object.html#size>
pub trait ValidateObjectMaxProperties {
    fn validate(&self, max_properties: usize) -> Result<(), MaxPropertiesErrorParams>;
}

impl<T> ValidateObjectMaxProperties for T
where
    T: Size,
{
    fn validate(&self, max_properties: usize) -> Result<(), MaxPropertiesErrorParams> {
        if max_properties >= self.size() {
            Ok(())
        } else {
            Err(MaxPropertiesErrorParams::new(max_properties))
        }
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
        assert!(ValidateObjectMaxProperties::validate(&map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_max_properties_btree_map_type() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(ValidateObjectMaxProperties::validate(&map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_max_properties_json_map_type() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(ValidateObjectMaxProperties::validate(map, 4).is_ok());
        assert!(ValidateObjectMaxProperties::validate(map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_max_properties_is_false() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(ValidateObjectMaxProperties::validate(map, 2).is_err());
    }
}
