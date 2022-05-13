use crate::{traits::Size, MinPropertiesErrorParams};

/// Min size validation of the object properties.
///
/// See <https://json-schema.org/understanding-json-schema/reference/object.html#size>
pub trait ValidateMinProperties {
    fn validate_min_properties(
        &self,
        min_properties: usize,
    ) -> Result<(), MinPropertiesErrorParams>;
}

impl<T> ValidateMinProperties for T
where
    T: Size,
{
    fn validate_min_properties(
        &self,
        min_properties: usize,
    ) -> Result<(), MinPropertiesErrorParams> {
        if min_properties <= self.size() {
            Ok(())
        } else {
            Err(MinPropertiesErrorParams::new(min_properties))
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
    fn test_validate_object_min_properties_hash_map_type() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(ValidateMinProperties::validate_min_properties(&map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_min_properties_btree_map_type() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        map.insert("key3".to_string(), "value3".to_string());
        assert!(ValidateMinProperties::validate_min_properties(&map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_min_properties_json_map_type() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(ValidateMinProperties::validate_min_properties(map, 2).is_ok());
        assert!(ValidateMinProperties::validate_min_properties(map, 3).is_ok());
    }

    #[test]
    fn test_validate_object_min_properties_is_false() {
        let value = json!({
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        });
        let map = value.as_object().unwrap();

        assert!(ValidateMinProperties::validate_min_properties(map, 4).is_err());
    }
}
