# Nested validation

To validate nested structures, add the `#[validate]` attribute to the target field.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct ParentStruct {
    #[validate] // <--- Add #[validate] attribute to the nested type field!
    nested: ChildStruct,
}

#[derive(Validate)]
struct ChildStruct {
    #[validate(maximum = 6)]
    val: i32,
}

assert!(
    ParentStruct {
        nested: ChildStruct{
            val: 5
        }
    }.validate().is_ok()
);
```
