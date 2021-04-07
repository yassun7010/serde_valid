use super::{inner_extract_array_unique_items_validator, VALIDATION_LABEL};
use crate::abort::abort_required_list_argument;
use crate::types::Field;
use crate::validator::common::{check_common_list_argument, extract_message_tokens};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_array_unique_items_validator_from_meta_list<F: Field>(
    field: &F,
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
            field.name(),
            field.ident(),
            attribute,
            validation_list,
        ))
    }
}

fn inner_extract_array_unique_items_validator_from_meta_list(
    field_name: &str,
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
) -> TokenStream {
    let syn::MetaList {
        nested: validation_args,
        ..
    } = validation_list;

    let message = extract_message_tokens(VALIDATION_LABEL, field_ident, attribute, validation_args)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::ItemsParams::to_default_message
        ));
    if validation_args.is_empty() && !check_common_list_argument(validation_list) {
        abort_required_list_argument(
            VALIDATION_LABEL,
            &["message_fn"],
            field_ident,
            attribute.span(),
            validation_list,
        )
    }
    inner_extract_array_unique_items_validator(field_name, field_ident, message)
}
