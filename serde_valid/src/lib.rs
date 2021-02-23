mod error;
mod validation;
pub use error::Error;
pub use validation::{validate_range, Limit};

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<self::Error>>;
}

#[cfg(feature = "derive")]
pub use serde_valid_derive::Validate;
