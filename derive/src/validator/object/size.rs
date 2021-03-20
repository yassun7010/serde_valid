use crate::helper::NamedField;
use crate::validator::common::extract_length_validator_tokens;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_object_size_validator(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_object_size_validator(
            &array_field,
            attribute,
            meta_items,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_object_size_validator(
            &option_field,
            attribute,
            meta_items,
        )))
    } else {
        Validator::Normal(inner_extract_object_size_validator(
            field.ident(),
            attribute,
            meta_items,
        ))
    }
}

fn inner_extract_object_size_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let field_string = field_ident.to_string();
    let (min_properties_tokens, max_properties_tokens) = extract_length_validator_tokens(
        field_ident,
        attribute,
        meta_items,
        "properties",
        "min_properties",
        "max_properties",
    );
    quote!(
        if !::serde_valid::validate_object_size(
            #field_ident,
            #min_properties_tokens,
            #max_properties_tokens
        ) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_insert(Vec::new())
                .push(
                    ::serde_valid::Error::PropertiesError(
                        ::serde_valid::error::PropertiesErrorMessage::new(
                            #field_ident,
                            #min_properties_tokens,
                            #max_properties_tokens
                        )
                    )
                );
        }
    )
}
