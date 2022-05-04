use super::{inner_extract_numeric_multiple_of_validator, VALIDATION_LABEL};
use crate::abort::{abort_duplicated_lit_argument, abort_invalid_attribute_on_field};
use crate::lit::LitNumeric;
use crate::types::Field;
use crate::validator::common::extract_message_tokens;
use crate::validator::common::{check_validation_arg_meta, get_numeric};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_numeric_multiple_of_validator_from_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
) -> Validator {
    let syn::MetaList {
        nested: validation_args,
        ..
    } = validation_list;
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(
            extract_numeric_multiple_of_validator_from_meta_list(
                &array_field,
                attribute,
                validation_list,
            ),
        ))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(
            extract_numeric_multiple_of_validator_from_meta_list(
                &option_field,
                attribute,
                validation_list,
            ),
        ))
    } else {
        Validator::Normal(inner_extract_numeric_multiple_of_validator_from_meta_list(
            field,
            attribute,
            validation_args,
        ))
    }
}

fn inner_extract_numeric_multiple_of_validator_from_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_args: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let multiple_of = get_multiple_of_from_meta_list(field, attribute, validation_args);
    let message = extract_message_tokens(VALIDATION_LABEL, field, attribute, validation_args)
        .unwrap_or(quote!(::serde_valid::MultipleOfParams::to_default_message));
    inner_extract_numeric_multiple_of_validator(field, multiple_of, message)
}

fn get_multiple_of_from_meta_list<'a>(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_args: &'a syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> LitNumeric<'a> {
    let mut multiple_of = None;
    for validation_arg in validation_args {
        match validation_arg {
            syn::NestedMeta::Lit(multiple_of_lit) => {
                if multiple_of.is_some() {
                    abort_duplicated_lit_argument(VALIDATION_LABEL, field, multiple_of_lit.span());
                }
                multiple_of = Some(get_numeric(VALIDATION_LABEL, field, multiple_of_lit));
            }
            syn::NestedMeta::Meta(arg_meta) => {
                check_validation_arg_meta(VALIDATION_LABEL, field, arg_meta, true)
            }
        }
    }
    multiple_of.unwrap_or_else(|| {
        abort_invalid_attribute_on_field(
            field,
            attribute.span(),
            "Validator `multiple_of` requires at least 1 argument from numeric literal",
        )
    })
}
