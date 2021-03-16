use crate::helper::NamedField;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_generic_enumerate_validator(
    field: &NamedField,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_generic_enumerate_validator(
            &array_field,
            meta_items,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_generic_enumerate_validator(
            &option_field,
            meta_items,
        )))
    } else {
        Validator::Normal(inner_extract_generic_enumerate_validator(
            field.ident(),
            meta_items,
        ))
    }
}

fn inner_extract_generic_enumerate_validator(
    field_ident: &syn::Ident,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let token = quote!(
        if !::serde_valid::validate_generic_enumerated_values(
            #field_ident,
            &[#meta_items],
        ) {
            errors.push(::serde_valid::Error::EnumeratedValuesError);
        }
    );
    token
}
