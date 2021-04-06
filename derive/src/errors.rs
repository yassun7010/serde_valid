use proc_macro2::TokenStream;
use quote::quote;

pub fn fields_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::Fields(
        ::serde_valid::validation::FieldsErrors::new(errors)
    ))
}

pub fn new_type_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::NewType(
        ::serde_valid::validation::NewTypeErrors::new(
            errors.remove(&::serde_valid::FieldName::new("0")).unwrap()
        )
    ))
}
