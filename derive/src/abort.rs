mod duplicated;
mod expected;
mod required;
mod unknown;

pub use duplicated::*;
pub use expected::*;
use proc_macro_error::abort;
pub use required::*;
pub use unknown::*;

pub fn abort_invalid_attribute_on_field(
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    message: &str,
) -> ! {
    abort!(
        span,
        "Invalid attribute #[validate] on field `{}`: {}",
        field_ident,
        message
    );
}
