# #[validate(exclusive_minimum = ???)]

The `#[validate(exclusive_minimum = ???)]` attribute is used to validate that a number is greater than a given value.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct SampleStruct {
    #[validate(exclusive_minimum = 2)]
    val: i32,
}

assert!(SampleStruct { val: 1 }.validate().is_err());
assert!(SampleStruct { val: 2 }.validate().is_err());
assert!(SampleStruct { val: 3 }.validate().is_ok());
```
