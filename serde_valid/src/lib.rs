mod error;
mod validation;
pub use error::Error;

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<self::Error>>;
}

#[cfg(feature = "derive")]
pub use serde_valid_derive::Validate;
