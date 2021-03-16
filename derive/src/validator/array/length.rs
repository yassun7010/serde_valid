use crate::helper::NamedField;
use crate::validator::common::extract_length_validator_tokens;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_array_length_validator(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_array_length_validator(
            &option_field,
            attribute,
            meta_items,
        )))
    } else {
        Validator::Normal(inner_extract_array_length_validator(
            field.ident(),
            attribute,
            meta_items,
        ))
    }
}

pub fn inner_extract_array_length_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let (min_items_tokens, max_items_tokens) = extract_length_validator_tokens(
        field_ident,
        attribute,
        meta_items,
        "items",
        "min_items",
        "max_items",
    );
    quote!(
        if !::serde_valid::validate_array_length(
            #field_ident,
            #min_items_tokens,
            #max_items_tokens
        ) {
            errors.push(::serde_valid::Error::ItemsError);
        }
    )
}
