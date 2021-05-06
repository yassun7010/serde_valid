use super::{inner_extract_string_pattern_validator, VALIDATION_LABEL};
use crate::abort::{abort_duplicated_lit_argument, abort_invalid_attribute_on_field};
use crate::types::Field;
use crate::validator::common::get_str;
use crate::validator::common::{check_validation_arg_meta, extract_message_tokens};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_string_pattern_of_validator_from_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(
            extract_string_pattern_of_validator_from_meta_list(
                &array_field,
                attribute,
                validation_list,
            ),
        ))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(
            extract_string_pattern_of_validator_from_meta_list(
                &option_field,
                attribute,
                validation_list,
            ),
        ))
    } else {
        Validator::Normal(inner_extract_string_pattern_of_validator_from_meta_list(
            field,
            attribute,
            validation_list,
        ))
    }
}

fn inner_extract_string_pattern_of_validator_from_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
) -> TokenStream {
    let syn::MetaList {
        nested: validation_args,
        ..
    } = validation_list;

    let pattern = get_pattern_from_meta_list(field, attribute, validation_args);
    let message = extract_message_tokens(VALIDATION_LABEL, field, attribute, validation_args)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::PatternParams::to_default_message
        ));
    inner_extract_string_pattern_validator(field, &pattern, &message)
}

fn get_pattern_from_meta_list<'a>(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_args: &'a syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> &'a syn::LitStr {
    let mut pattern = None;

    for validation_arg in validation_args {
        match validation_arg {
            syn::NestedMeta::Lit(pattern_lit) => {
                if pattern.is_some() {
                    abort_duplicated_lit_argument(VALIDATION_LABEL, field, pattern_lit.span());
                }
                pattern = Some(get_str(VALIDATION_LABEL, field, pattern_lit));
            }
            syn::NestedMeta::Meta(meta) => {
                check_validation_arg_meta(VALIDATION_LABEL, field, meta, true)
            }
        }
    }
    pattern.unwrap_or_else(|| {
        abort_invalid_attribute_on_field(
            field,
            attribute.span(),
            &format!(
                "Validator `{}` requires at least 1 argument from str literal",
                VALIDATION_LABEL
            ),
        )
    })
}
