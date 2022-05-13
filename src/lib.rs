//! # Serde Valid
//!
//! [![Latest Version](https://img.shields.io/crates/v/serde_valid.svg?color=green&style=flat-square)](https://crates.io/crates/serde_valid)
//! [![GitHub license](https://badgen.net/github/license/Naereen/Strapdown.js?style=flat-square)](https://github.com/Naereen/StrapDown.js/blob/master/LICENSE)
//!
//! This is [JSON Schema](https://json-schema.org/) based validation tool using with [serde](https://github.com/serde-rs/serde).
//!
//! ## Usage
//!
//! You derive `Validate` trait, and write validations.
//!
//! ```rust
//! use serde_valid::Validate;
//!
//! #[derive(Validate)]
//! struct SampleStruct {
//!     #[validate(minimum = 0)]
//!     #[validate(maximum = 10)]
//!     val: i32,
//! }
//!
//! #[derive(Validate)]
//! enum SampleEnum {
//!     Named {
//!         #[validate]
//!         a: SampleStruct,
//!     },
//!     UnNamed(
//!         #[validate(minimum = 0)]
//!         #[validate(maximum = 10)]
//!         i32,
//!     ),
//! }
//!
//! let s = SampleEnum::Named {
//!     a: SampleStruct { val: 5 },
//! };
//!
//! assert!(s.validate().is_ok());
//! ```
//!
//! ## Validations
//!
//! Serde Valid support standard validation based JSON Schema.
//!
//! | Type | Serde Valid(validate derive) | Json Schema |
//! | :---: | :--- | :--- |
//! | String | `#[validate(max_length = 5)]` | [maxLength](https://json-schema.org/understanding-json-schema/reference/string.html#length) |
//! | String | `#[validate(min_length = 5)]` | [minLength](https://json-schema.org/understanding-json-schema/reference/string.html#length) |
//! | String | `#[validate(pattern = r"^\d{5}$")]` | [pattern](https://json-schema.org/understanding-json-schema/reference/string.html#regular-expressions) |
//! | Numeric | `#[validate(maximum = 5)]` | [maximum](https://json-schema.org/understanding-json-schema/reference/numeric.html#range) |
//! | Numeric | `#[validate(minimum = 5)]` | [minimum](https://json-schema.org/understanding-json-schema/reference/numeric.html#range) |
//! | Numeric | `#[validate(exclusive_maximum = 5)]` | [exclusiveMaximum](https://json-schema.org/understanding-json-schema/reference/numeric.html#range) |
//! | Numeric | `#[validate(exclusive_minimum = 5)]` | [exclusiveMinimum](https://json-schema.org/understanding-json-schema/reference/numeric.html#range) |
//! | Numeric | `#[validate(multiple_of = 5)]` | [multipleOf](https://json-schema.org/understanding-json-schema/reference/numeric.html#multiples) |
//! | Object | `#[validate(max_properties = 5)]` | [maxProperties](https://json-schema.org/understanding-json-schema/reference/object.html#size) |
//! | Object | `#[validate(min_properties = 5)]` | [minProperties](https://json-schema.org/understanding-json-schema/reference/object.html#size) |
//! | Array | `#[validate(max_items = 5)]` | [maxItems](https://json-schema.org/understanding-json-schema/reference/array.html#length) |
//! | Array | `#[validate(min_items = 5)]` | [minItems](https://json-schema.org/understanding-json-schema/reference/array.html#length) |
//! | Array | `#[validate(unique_items)]` | [uniqueItems](https://json-schema.org/understanding-json-schema/reference/array.html#unique_items) |
//! | Generic | `#[validate(enumerate(5, 10, 15))]` | [enum](https://json-schema.org/understanding-json-schema/reference/generic.html#enumerated-values) |
//!
//! ## Complete Constructor (Deserialization)
//!
//! Serde Valid support complete constructor method using by [`serde_valid::json::FromJson`](json::FromJson) trait.
//!
//! ```rust
//! use serde::Deserialize;
//! use serde_valid::Validate;
//! use serde_valid::json::{json, FromJson};
//!
//! #[derive(Debug, Deserialize, Validate)]
//! struct SampleStruct {
//!     #[validate(maximum = 100)]
//!     val: i32,
//! }
//!
//! // Deserialization and Validation!! 🚀
//! let err = SampleStruct::from_json_value(json!({ "val": 123 })).unwrap_err();
//!
//! assert_eq!(
//!     serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
//!     json!({ "val": [ "the number must be `<= 100`." ] })
//! );
//! ```
//!
//! You can force validation by only deserialization through `serde_valid`, and removing `serde_json` from `Cargo.toml` of your project.
//!
//! ## Serialization
//!
//! For serialization, provides [`serde_valid::json::ToJson`](json::ToJson) trait.
//!
//! ```rust
//! use serde::Serialize;
//! use serde_valid::Validate;
//! use serde_valid::json::{json, ToJson};
//!
//! #[derive(Debug, Serialize, Validate)]
//! struct SampleStruct {
//!     #[validate(maximum = 100)]
//!     val: i32,
//! }
//!
//! assert_eq!(
//!     SampleStruct{ val: 12i32 }.to_json_string().unwrap(),
//!     json!({ "val": 12i32 }).to_json_string().unwrap()
//! );
//! ```
//!
//! ## Custom Message
//!
//! For user custom message, Serde Valid provides `message_fn` or `message`.
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//!
//! fn min_error_message(_params: &serde_valid::MinItemsErrorParams) -> String {
//!     "this is min custom message_fn.".to_string()
//! }
//!
//! #[derive(Validate)]
//! struct SampleStruct {
//!     #[validate(min_items = 4, message_fn(min_error_message))]
//!     #[validate(max_items = 2, message = "this is max custom message.")]
//!     val: Vec<i32>,
//! }
//!
//! let s = SampleStruct { val: vec![1, 2, 3] };
//!
//! assert_eq!(
//!     serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
//!     serde_json::to_string(&json!({
//!         "val": [
//!             "this is min custom message_fn.",
//!             "this is max custom message."
//!         ]
//!     }))
//!     .unwrap()
//! );
//! ```
//!
//! ## Custom method
//!
//! You can use your custom validation using by `#[validate(custom)]`.
//!
//! ```rust
//! use serde_valid::Validate;
//!
//! fn user_validation(_val: &i32) -> Result<(), serde_valid::validation::Error> {
//!     Ok(())
//! }
//!
//! #[derive(Validate)]
//! struct SampleStruct {
//!     #[validate(custom(user_validation))]
//!     val: i32,
//! }
//!
//! let s = SampleStruct { val: 1 };
//!
//! assert!(s.validate().is_ok());
//! ```
//!
//! ## Rules
//!
//! If you want to check multi fields validation, you can use `#[rule]`.
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//!
//! fn sample_rule(_val1: &i32, _val2: &str) -> Result<(), serde_valid::validation::Error> {
//!     Err(serde_valid::validation::Error::Custom(
//!         "Rule error is added to the first arg of the rule_method.".to_owned(),
//!     ))
//! }
//!
//! #[derive(Validate)]
//! #[rule(sample_rule(val2, val1))]
//! struct SampleStruct {
//!     val1: String,
//!     val2: i32,
//! }
//!
//! let s = SampleStruct {
//!     val1: "val1".to_owned(),
//!     val2: 1,
//! };
//!
//! assert_eq!(
//!     serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
//!     serde_json::to_string(&json!({
//!         "val2": [
//!             "Rule error is added to the first arg of the rule_method."
//!         ]
//!     }))
//!     .unwrap()
//! );
//! ```
//!
//! If you want to use rule to unnamed fields struct, just like this,
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//!
//! fn sample_rule(_val1: &i32, _val2: &str) -> Result<(), serde_valid::validation::Error> {
//!     Ok(())
//! }
//!
//! #[derive(Validate)]
//! #[rule(sample_rule(0, 1))]
//! struct SampleStruct(i32, String);
//!
//! let s = SampleStruct(0, "1".to_owned());
//!
//! assert!(s.validate().is_ok());
//! ```
//!
//! ## Validate Traits
//!
//! By implementing the validation trait, Your original type can uses Serde Valid validations.
//!
//! ```rust
//! use serde_valid::Validate;
//!
//! struct MyType(String);
//!
//! impl serde_valid::ValidateMaxLength for MyType {
//!     fn validate_max_length(&self, max_length: usize) -> Result<(), serde_valid::MaxLengthErrorParams> {
//!         self.0.validate_max_length(max_length)
//!     }
//! }
//!
//! impl serde_valid::ValidateMinLength for MyType {
//!     fn validate_min_length(&self, min_length: usize) -> Result<(), serde_valid::MinLengthErrorParams> {
//!         self.0.validate_min_length(min_length)
//!     }
//! }
//!
//! #[derive(Validate)]
//! struct SampleStruct {
//!     #[validate(min_length = 5)]
//!     #[validate(max_length = 5)]
//!     val: MyType,
//! }
//!
//! let s = SampleStruct {
//!     val: MyType(String::from("😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦")),
//! };
//!
//! assert!(s.validate().is_ok());
//! ```

pub mod error;
mod traits;
pub mod validation;

pub use error::{
    EnumerateErrorParams, Error, ExclusiveMaximumErrorParams, ExclusiveMinimumErrorParams,
    MaxItemsErrorParams, MaxLengthErrorParams, MaxPropertiesErrorParams, MaximumErrorParams,
    MinItemsErrorParams, MinLengthErrorParams, MinPropertiesErrorParams, MinimumErrorParams,
    MultipleOfErrorParams, PatternErrorParams, UniqueItemsErrorParams,
};
pub use validation::{
    ValidateEnumerate, ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaxItems,
    ValidateMaxLength, ValidateMaxProperties, ValidateMaximum, ValidateMinItems, ValidateMinLength,
    ValidateMinProperties, ValidateMinimum, ValidateMultipleOf, ValidatePattern,
    ValidateUniqueItems,
};

pub trait Validate {
    fn validate(&self) -> std::result::Result<(), self::validation::Errors>;
}

pub use serde_valid_derive::Validate;

pub mod json;
#[cfg(feature = "toml")]
pub mod toml;
#[cfg(feature = "yaml")]
pub mod yaml;
