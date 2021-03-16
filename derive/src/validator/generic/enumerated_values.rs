use crate::helper::NamedField;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_generic_enumerate_validator(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_generic_enumerate_validator(
            &array_field,
            attribute,
            meta_items,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_generic_enumerate_validator(
            &option_field,
            attribute,
            meta_items,
        )))
    } else {
        Validator::Normal(inner_extract_generic_enumerate_validator(
            field.ident(),
            attribute,
            meta_items,
        ))
    }
}

fn inner_extract_generic_enumerate_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    if meta_items.len() == 0 {
        abort!(
            attribute.span(),
            "'enumerate' meta_items size must be greater than 0."
        )
    }
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
