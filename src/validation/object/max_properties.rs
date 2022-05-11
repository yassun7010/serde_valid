use crate::{traits::Size, MaxPropertiesErrorParams};

/// Max size of the object properties validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/object.html#size>
pub trait ValidateMaxProperties {
    fn validate_max_properties(
        &self,
        max_properties: usize,
    ) -> Result<(), MaxPropertiesErrorParams>;
}

impl<T> ValidateMaxProperties for T
where
    T: Size,
{
    fn validate_max_properties(
        &self,
        max_properties: usize,
    ) -> Result<(), MaxPropertiesErrorParams> {
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
        assert!(ValidateMaxProperties::validate_max_properties(&map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_max_properties_btree_map_type() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(ValidateMaxProperties::validate_max_properties(&map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_max_properties_json_map_type() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(ValidateMaxProperties::validate_max_properties(map, 4).is_ok());
        assert!(ValidateMaxProperties::validate_max_properties(map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_max_properties_is_false() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(ValidateMaxProperties::validate_max_properties(map, 2).is_err());
    }
}
