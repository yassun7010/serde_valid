# Numeric: "multiple_of" validation

The `#[validate(multiple_of = ???)]` attribute is used to validate that a number is a multiple of given number.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct Data (
    #[validate(multiple_of = 5)]
    i32,
);

assert!(Data(15).validate().is_ok());
assert!(Data(14).validate().is_err());
```
