#[cfg(not(feature = "serde_error"))]
mod default_error;
#[cfg(not(feature = "serde_error"))]
pub use default_error::{from_reader, from_slice, from_str, from_value};

#[cfg(feature = "serde_error")]
mod serde_error;
#[cfg(feature = "serde_error")]
pub use serde_error::{from_reader, from_slice, from_str, from_value};
