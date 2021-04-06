use proc_macro2::TokenStream;
use quote::quote;

pub fn fields_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::Fields(errors))
}

pub fn new_type_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::NewType(
        errors.remove(&::serde_valid::FieldName::new("0")).unwrap()
    ))
}
