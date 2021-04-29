#[cfg(not(feature = "serde_error"))]
mod default_error;
#[cfg(feature = "serde_error")]
mod serde_error;

#[cfg(not(feature = "serde_error"))]
pub use default_error::*;
#[cfg(feature = "serde_error")]
pub use serde_error::*;
