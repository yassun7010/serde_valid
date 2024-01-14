# String: "max_length" validation

The `#[validate(max_length = ???)]` attribute is used to validate that a `String` is no longer than a given length.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct Data (
    #[validate(max_length = 4)]
    String,
);

assert!(Data("test".to_owned()).validate().is_ok());
assert!(Data("test1".to_owned()).validate().is_err());
```
