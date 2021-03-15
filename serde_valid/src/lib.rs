mod error;
mod traits;
mod validation;
pub use error::Error;
pub use validation::{
    validate_array_length, validate_array_uniqueness, validate_number_multiples,
    validate_number_range, validate_string_length, validate_string_pattern, Limit,
};

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<self::Error>>;
}

#[cfg(feature = "derive")]
pub use serde_valid_derive::Validate;
