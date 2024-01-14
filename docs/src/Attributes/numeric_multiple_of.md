# "multiple_of" validation

The `#[validate(multiple_of = ???)]` attribute is used to validate that a number is a multiple of given number.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct TestStruct {
    #[validate(multiple_of = 5)]
    val: i32,
}

assert!(TestStruct { val: 15 }.validate().is_ok());
assert!(TestStruct { val: 14 }.validate().is_err());
```
