use super::{inner_extract_numeric_multiple_of_validator, VALIDATION_LABEL};
use crate::abort::{abort_duplicated_lit_argument, abort_invalid_attribute_on_field};
use crate::lit::LitNumeric;
use crate::types::NamedField;
use crate::validator::common::extract_message_tokens;
use crate::validator::common::{check_meta, get_numeric};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_numeric_multiple_of_validator_from_meta_list(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Validator {
    let syn::MetaList { nested, .. } = meta_list;
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(
            extract_numeric_multiple_of_validator_from_meta_list(
                &array_field,
                attribute,
                meta_list,
            ),
        ))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(
            extract_numeric_multiple_of_validator_from_meta_list(
                &option_field,
                attribute,
                meta_list,
            ),
        ))
    } else {
        Validator::Normal(inner_extract_numeric_multiple_of_validator_from_meta_list(
            field.ident(),
            attribute,
            nested,
        ))
    }
}

fn inner_extract_numeric_multiple_of_validator_from_meta_list(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let multiple_of = get_multiple_of_from_meta_list(field_ident, attribute, meta_items);
    let message = extract_message_tokens(VALIDATION_LABEL, field_ident, attribute, meta_items)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::MultipleOfParams::to_default_message
        ));
    inner_extract_numeric_multiple_of_validator(field_ident, multiple_of, message)
}

fn get_multiple_of_from_meta_list(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> LitNumeric {
    let mut multiple_of = None;
    for meta in meta_items {
        match meta {
            syn::NestedMeta::Lit(lit) => {
                if multiple_of.is_some() {
                    abort_duplicated_lit_argument(VALIDATION_LABEL, field_ident, lit.span());
                }
                multiple_of = Some(get_numeric(VALIDATION_LABEL, field_ident, lit));
            }
            syn::NestedMeta::Meta(meta) => {
                check_meta(VALIDATION_LABEL, field_ident, meta.span(), meta, true)
            }
        }
    }
    multiple_of.unwrap_or_else(|| {
        abort_invalid_attribute_on_field(
            field_ident,
            attribute.span(),
            "Validator `multiple_of` requires at least 1 argument from numeric literal",
        )
    })
}
