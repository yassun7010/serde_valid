use proc_macro2::TokenStream;
use quote::quote;

pub fn fields_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::Fields(
        ::serde_valid::validation::FieldsErrors::new(errors)
    ))
}

pub fn single_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::Single(
        ::serde_valid::validation::SingleErrors::new(
            errors.remove(&::serde_valid::FieldName::new("0")).unwrap()
        )
    ))
}
