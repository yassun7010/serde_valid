# "maximum" validation

The `#[validate(maximum = ???)]` attribute is used to ensure that a value is less than or equal to a given value.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct SampleStruct {
    #[validate(maximum = 6)]
    val: i32,
}

assert!(SampleStruct { val: 5 }.validate().is_ok());
assert!(SampleStruct { val: 6 }.validate().is_ok());
assert!(SampleStruct { val: 7 }.validate().is_err());
```
