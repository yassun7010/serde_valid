use crate::types::Field;
use crate::validator::common::{extract_length_validator_tokens, extract_message_tokens};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

const VALIDATION_LABEL: &'static str = "properties";
const MIN_LABEL: &'static str = "min_properties";
const MAX_LABEL: &'static str = "max_properties";

pub fn extract_object_properties_validator<F: Field>(
    field: &F,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Validator {
    let syn::MetaList { nested, .. } = meta_list;

    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_object_properties_validator(
            &array_field,
            attribute,
            meta_list,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_object_properties_validator(
            &option_field,
            attribute,
            meta_list,
        )))
    } else {
        Validator::Normal(inner_extract_object_properties_validator(
            field.name(),
            field.ident(),
            attribute,
            nested,
        ))
    }
}

fn inner_extract_object_properties_validator(
    field_name: &str,
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let (min_properties_tokens, max_properties_tokens) = extract_length_validator_tokens(
        VALIDATION_LABEL,
        MIN_LABEL,
        MAX_LABEL,
        field_ident,
        attribute,
        meta_items,
    );
    let message = extract_message_tokens(VALIDATION_LABEL, field_ident, attribute, meta_items)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::PropertiesParams::to_default_message
        ));

    quote!(
        if !::serde_valid::validate_object_properties(
            #field_ident,
            #min_properties_tokens,
            #max_properties_tokens
        ) {
            use ::serde_valid::validation::error::ToDefaultMessage;
            errors
                .entry(::serde_valid::FieldName::new(#field_name))
                .or_default()
                .push(::serde_valid::validation::Error::Properties(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::PropertiesParams::new(
                            #field_ident,
                            #min_properties_tokens,
                            #max_properties_tokens
                        ),
                        #message
                    )
                ));
        }
    )
}
