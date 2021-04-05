use super::{inner_extract_string_pattern_validator, VALIDATION_LABEL};
use crate::abort::{abort_duplicated_lit_argument, abort_invalid_attribute_on_field};
use crate::types::Field;
use crate::validator::common::get_str;
use crate::validator::common::{check_meta, extract_message_tokens};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_string_pattern_of_validator_from_meta_list<F: Field>(
    field: &F,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(
            extract_string_pattern_of_validator_from_meta_list(&array_field, attribute, meta_list),
        ))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(
            extract_string_pattern_of_validator_from_meta_list(&option_field, attribute, meta_list),
        ))
    } else {
        Validator::Normal(inner_extract_string_pattern_of_validator_from_meta_list(
            field.name(),
            field.ident(),
            attribute,
            meta_list,
        ))
    }
}

fn inner_extract_string_pattern_of_validator_from_meta_list(
    field_name: &str,
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> TokenStream {
    let syn::MetaList { nested, .. } = meta_list;

    let pattern = get_pattern_from_meta_list(field_ident, attribute, meta_list);
    let message = extract_message_tokens(VALIDATION_LABEL, field_ident, attribute, nested)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::PatternParams::to_default_message
        ));
    inner_extract_string_pattern_validator(field_name, field_ident, &pattern, &message)
}

fn get_pattern_from_meta_list(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> syn::LitStr {
    let syn::MetaList { nested, .. } = meta_list;

    let mut pattern = None;

    for meta in nested {
        match meta {
            syn::NestedMeta::Lit(lit) => {
                if pattern.is_some() {
                    abort_duplicated_lit_argument(VALIDATION_LABEL, field_ident, lit.span());
                }
                pattern = Some(get_str(VALIDATION_LABEL, field_ident, lit));
            }
            syn::NestedMeta::Meta(meta) => {
                check_meta(VALIDATION_LABEL, field_ident, meta.span(), meta, true)
            }
        }
    }
    pattern.unwrap_or_else(|| {
        abort_invalid_attribute_on_field(
            field_ident,
            attribute.span(),
            &format!(
                "Validator `{}` requires at least 1 argument from str literal",
                VALIDATION_LABEL
            ),
        )
    })
}
