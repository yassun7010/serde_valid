mod deserialize;
mod traits;
pub mod validation;

pub use deserialize::*;
pub use traits::*;
pub use validation::{
    validate_array_items, validate_array_unique_items, validate_generic_enumerate,
    validate_numeric_multiple_of, validate_numeric_range, validate_object_properties,
    validate_string_length, validate_string_pattern, Limit,
};

#[cfg(not(serde_error))]
mod error;
#[cfg(not(serde_error))]
pub use error::Error;

pub trait Validate {
    fn validate(&self) -> Result<(), self::validation::Errors>;
}

pub use serde_valid_derive::Validate;
