# Number: "exclusive_maximum" validation

The `#[validate(exclusive_maximum = ???)]` attribute is used to validate that a number is greater than a given value.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct SampleStruct {
    #[validate(exclusive_maximum = 6)]
    val: i32,
}

assert!(SampleStruct { val: 5 }.validate().is_ok());
assert!(SampleStruct { val: 6 }.validate().is_err());
assert!(SampleStruct { val: 7 }.validate().is_err());
```