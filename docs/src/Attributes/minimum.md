# #[validate(minimum)]

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct SampleStruct {
    #[validate(minimum = 2)]
    val: i32,
}

assert!(SampleStruct { val: 1 }.validate().is_err());
assert!(SampleStruct { val: 2 }.validate().is_ok());
```
