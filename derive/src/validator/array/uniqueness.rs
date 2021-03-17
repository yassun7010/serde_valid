use crate::helper::NamedField;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_array_uniqueness_validator(field: &NamedField) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_array_uniqueness_validator(&option_field)))
    } else {
        Validator::Normal(inner_extract_array_uniqueness_validator(field.ident()))
    }
}

fn inner_extract_array_uniqueness_validator(field_ident: &syn::Ident) -> TokenStream {
    quote!(
        if !::serde_valid::validate_array_uniqueness(
            #field_ident
        ) {
            errors.push(::serde_valid::Error::UniqueItemsError);
        }
    )
}
