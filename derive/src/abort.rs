mod duplicated;
mod expected;
mod required;
mod unknown;

use crate::types::Field;
pub use duplicated::*;
pub use expected::*;
use proc_macro_error::abort;
pub use required::*;
pub use unknown::*;

pub fn abort_invalid_attribute_on_field(
    field: &impl Field,
    span: proc_macro2::Span,
    message: &str,
) -> ! {
    abort!(
        span,
        "Invalid attribute #[validate] on field `{}`: {}",
        field.ident(),
        message
    );
}
