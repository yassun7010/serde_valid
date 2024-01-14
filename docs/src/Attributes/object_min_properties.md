# Object: "min_properties" validation

The `#[validate(min_properties = ???)]` attribute is used to determine the minimum number of properties allowed in a map.

```rust
# extern crate serde_valid;
use serde_valid::Validate;
use std::collections::HashMap;

#[derive(Validate)]
struct Data(
    #[validate(min_properties = 3)]
    HashMap<String, String>,
);

let mut map = HashMap::new();
map.insert("key1".to_string(), "value1".to_string());
map.insert("key2".to_string(), "value2".to_string());

assert!(Data(map.clone()).validate().is_err());

map.insert("key3".to_string(), "value3".to_string());

assert!(Data(map.clone()).validate().is_ok());
```