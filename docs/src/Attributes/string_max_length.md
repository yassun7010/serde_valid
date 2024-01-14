# String: "max_length" validation

The `#[validate(max_length = ???)]` attribute is used to validate that a `String` is no longer than a given length.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct TestStruct {
    #[validate(max_length = 4)]
    val: String,
}

assert!(TestStruct { val: "test".to_owned() }.validate().is_ok());
assert!(TestStruct { val: "test1".to_owned() }.validate().is_err());
```
