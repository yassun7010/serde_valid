mod error;
mod traits;
mod validation;
pub use error::Error;
pub use validation::{validate_length, validate_multiples, validate_range, Limit};

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<self::Error>>;
}

#[cfg(feature = "derive")]
pub use serde_valid_derive::Validate;
