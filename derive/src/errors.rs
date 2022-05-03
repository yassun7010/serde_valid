use proc_macro2::TokenStream;
use quote::quote;

use crate::types::Field;

pub fn fields_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::Fields(__errors))
}

pub fn new_type_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::NewType(
        __errors.remove("0").unwrap()
    ))
}

#[derive(Debug)]
pub struct Error(syn::Error);

impl Error {
    pub fn new<Message: Into<String>>(span: proc_macro2::Span, message: Message) -> Self {
        Self(syn::Error::new(span, message.into()))
    }

    pub fn new_invalid_field_attribute_error(
        field: &impl Field,
        span: proc_macro2::Span,
        message: &str,
    ) -> Self {
        Self::new(
            span,
            format!(
                "Invalid attribute #[validate] on field `{indent}`: {message}",
                indent = field.ident()
            ),
        )
    }

    pub fn new_literal_meta_item_error(span: proc_macro2::Span) -> Self {
        Self::new(span, "literal meta item does not support.")
    }

    pub fn new_attribute_parse_error(span: proc_macro2::Span, error: &syn::Error) -> Self {
        Self::new(span, format!("attribute parse error: {error}"))
    }

    pub fn to_compile_error(&self) -> TokenStream {
        self.0.to_compile_error()
    }
}

pub type Errors = Vec<Error>;

pub fn to_compile_errors(errors: Errors) -> TokenStream {
    let compile_errors = errors.iter().map(Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
