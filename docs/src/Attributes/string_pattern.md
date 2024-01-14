# String: "pattern" validation

The `#[validate(pattern = ???)]` attribute is used to validate a string against a regular expression.

```rust
# extern crate regex;
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct Data (
    #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
    String,
);

let s = Data("2020-09-10".to_owned());
assert!(s.validate().is_ok());
```
