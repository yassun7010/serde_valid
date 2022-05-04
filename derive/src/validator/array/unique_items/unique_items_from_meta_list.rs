use super::{inner_extract_array_unique_items_validator, VALIDATION_LABEL};
use crate::abort::abort_unknown_list_argument;
use crate::types::Field;
use crate::validator::common::extract_message_tokens;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_array_unique_items_validator_from_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(
            extract_array_unique_items_validator_from_meta_list(
                &option_field,
                attribute,
                validation_list,
            ),
        ))
    } else {
        Validator::Normal(inner_extract_array_unique_items_validator_from_meta_list(
            field,
            attribute,
            validation_list,
        ))
    }
}

fn inner_extract_array_unique_items_validator_from_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
) -> TokenStream {
    let syn::MetaList {
        nested: validation_args,
        ..
    } = validation_list;

    let message = extract_message_tokens(VALIDATION_LABEL, field, attribute, validation_args);

    if !validation_args.is_empty() && message.is_none() {
        abort_unknown_list_argument(VALIDATION_LABEL, field, attribute.span(), validation_list)
    }

    inner_extract_array_unique_items_validator(
        field,
        message.unwrap_or(quote!(
            ::serde_valid::validation::ItemsParams::to_default_message
        )),
    )
}
