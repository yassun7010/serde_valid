# String: "min_length" validation

The `#[validate(min_length = ???)]` attribute is used to validate that a `String` is no longer than a given length.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct TestStruct {
    #[validate(min_length = 4)]
    val: String,
}

assert!(TestStruct { val: "tes".to_owned() }.validate().is_err());
assert!(TestStruct { val: "test".to_owned() }.validate().is_ok());
```
