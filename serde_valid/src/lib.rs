pub mod error;
mod traits;
mod validation;
pub use error::Error;
pub use validation::{
    validate_array_length, validate_array_uniqueness, validate_generic_enumerated_values,
    validate_numeric_multiples, validate_numeric_range, validate_object_size,
    validate_string_length, validate_string_regular_expressions, Limit,
};

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<self::Error>>;
}

#[cfg(feature = "derive")]
pub use serde_valid_derive::Validate;
