# Array: "max_items" validation

The `#[validate(max_items = ???)]` attribute is used to determine the maximum number of items in an array.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct Data(
    #[validate(max_items = 2)]
    Vec<u8>,
);

assert!(Data(vec![1, 2]).validate().is_ok());
assert!(Data(vec![1, 2, 3]).validate().is_err());
```
