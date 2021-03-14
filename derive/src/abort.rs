use proc_macro_error::abort;

pub fn abort_unnamed_fields_struct(field_span: proc_macro2::Span) -> ! {
    abort!(
        field_span,
        "struct has unnamed fields";
        help = "#[derive(Validate)] can only be used on structs with named fields";
    )
}

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
