# Numeric: "maximum" validation

The `#[validate(maximum = ???)]` attribute is used to ensure that a value is less than or equal to a given value.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct Data (
    #[validate(maximum = 6)]
    i32
);

assert!(Data(5).validate().is_ok());
assert!(Data(6).validate().is_ok());
assert!(Data(7).validate().is_err());
```
