use proc_macro_error::abort;

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
