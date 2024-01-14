# Numeric: "exclusive_minimum" validation

The `#[validate(exclusive_minimum = ???)]` attribute is used to validate that a number is greater than a given value.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct Data (
    #[validate(exclusive_minimum = 2)]
    i32
);

assert!(Data(1).validate().is_err());
assert!(Data(2).validate().is_err());
assert!(Data(3).validate().is_ok());
```
