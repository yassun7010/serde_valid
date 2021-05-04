use crate::types::Field;
use crate::validator::common::{extract_length_validator_tokens, extract_message_tokens};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

const VALIDATION_LABEL: &'static str = "items";
const MIN_LABEL: &'static str = "min_items";
const MAX_LABEL: &'static str = "max_items";

pub fn extract_array_items_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
) -> Validator {
    let syn::MetaList {
        nested: validation_args,
        ..
    } = validation_list;

    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_array_items_validator(
            &option_field,
            attribute,
            validation_list,
        )))
    } else {
        Validator::Normal(inner_extract_array_items_validator(
            field,
            attribute,
            validation_args,
        ))
    }
}

fn inner_extract_array_items_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_args: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let field_name = field.name();
    let field_ident = field.ident();
    let (min_items_tokens, max_items_tokens) = extract_length_validator_tokens(
        VALIDATION_LABEL,
        MIN_LABEL,
        MAX_LABEL,
        field,
        attribute,
        validation_args,
    );
    let message = extract_message_tokens(VALIDATION_LABEL, field, attribute, validation_args)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::ItemsParams::to_default_message
        ));

    quote!(
        if !::serde_valid::validate_array_items(
            #field_ident,
            #min_items_tokens,
            #max_items_tokens
        ) {
            use ::serde_valid::validation::error::ToDefaultMessage;
            __errors
                .entry(#field_name)
                .or_default()
                .push(::serde_valid::validation::Error::Items(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::ItemsParams::new(
                            #field_ident,
                            #min_items_tokens,
                            #max_items_tokens
                        ),
                        #message
                    )
                ));
        }
    )
}
