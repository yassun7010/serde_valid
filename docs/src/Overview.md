<span style="float:right">
  [![GitHub]][repo]
  [![rustdoc]][docs]
  [![Latest Version]][crates.io]
</span>

[GitHub]: /img/github.svg
[repo]: https://github.com/serde-rs/serde
[rustdoc]: /img/rustdoc.svg
[docs]: https://docs.rs/serde
[Latest Version]: https://img.shields.io/crates/v/serde.svg?style=social
[crates.io]: https://crates.io/crates/serde

<div style="clear:both"></div>

# Overview
[![GitHub](/img/github.svg)](https://github.com/yassun7010/serde_valid)
[![Documentation](/img/rustdoc.svg)](https://docs.rs/serde_valid)
[![Latest Version](https://img.shields.io/crates/v/serde_valid.svg?style=social)](https://crates.io/crates/serde_valid)

This is [JSON Schema](https://json-schema.org/) based validation tool using with [serde](https://github.com/serde-rs/serde).

## Usage

You derive `Validate` trait, and write validations.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct SampleStruct {
    #[validate(minimum = 0)]
    #[validate(maximum = 10)]
    val: i32,
}

#[derive(Validate)]
enum SampleEnum {
    Named {
        #[validate]
        a: SampleStruct,
    },
}

let s = SampleEnum::Named {
    a: SampleStruct { val: 5 },
};

assert!(s.validate().is_ok());
```
