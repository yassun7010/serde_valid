# Numeric: "minimum" validation

The `#[validate(minimum = ???)]` attribute is used to validate that a field is greater than or equal to a given value.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct Data (
    #[validate(minimum = 2)]
    i32,
);

assert!(Data(1).validate().is_err());
assert!(Data(2).validate().is_ok());
```
