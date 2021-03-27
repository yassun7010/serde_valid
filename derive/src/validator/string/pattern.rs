use crate::abort::{abort_duplicated_lit_argument, abort_invalid_attribute_on_field};
use crate::helper::NamedField;
use crate::validator::common::get_str;
use crate::validator::common::{check_meta, extract_message_tokens};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

const VALIDATION_LABEL: &'static str = "pattern";

pub fn extract_string_pattern_validator_from_name_value(
    field: &NamedField,
    lit: &syn::Lit,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_string_pattern_validator_from_name_value(
            &array_field,
            lit,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_string_pattern_validator_from_name_value(
            &option_field,
            lit,
        )))
    } else {
        Validator::Normal(inner_extract_string_pattern_validator_from_name_value(
            field.ident(),
            lit,
        ))
    }
}

pub fn extract_string_pattern_of_validator_from_list(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_string_pattern_of_validator_from_list(
            &array_field,
            attribute,
            meta_list,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_string_pattern_of_validator_from_list(
            &option_field,
            attribute,
            meta_list,
        )))
    } else {
        Validator::Normal(inner_extract_string_pattern_of_validator_from_list(
            field.ident(),
            attribute,
            meta_list,
        ))
    }
}

fn inner_extract_string_pattern_of_validator_from_list(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> TokenStream {
    let syn::MetaList { nested, .. } = meta_list;

    let pattern = get_pattern_from_list(field_ident, attribute, meta_list);
    let message = extract_message_tokens(VALIDATION_LABEL, field_ident, attribute, nested)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::RegularExpressionErrorParams::to_default_message
        ));
    inner_extract_string_pattern_validator(field_ident, &pattern, &message)
}

fn inner_extract_string_pattern_validator_from_name_value(
    field_ident: &syn::Ident,
    lit: &syn::Lit,
) -> TokenStream {
    let pattern = get_str(VALIDATION_LABEL, field_ident, lit);
    let message =
        quote!(::serde_valid::validation::error::RegularExpressionErrorParams::to_default_message);
    inner_extract_string_pattern_validator(field_ident, &pattern, &message)
}

fn inner_extract_string_pattern_validator(
    field_ident: &syn::Ident,
    pattern: &syn::LitStr,
    message: &TokenStream,
) -> TokenStream {
    let field_string = field_ident.to_string();
    let pattern_ident = syn::Ident::new(
        &format!("{}_PATTERN", &field_ident).to_uppercase(),
        field_ident.span(),
    );

    quote!(
        static #pattern_ident : once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        let pattern = #pattern_ident.get_or_init(|| regex::Regex::new(#pattern).unwrap());
        if !::serde_valid::validate_string_regular_expressions(
            #field_ident,
            pattern,
        ) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_default()
                .push(::serde_valid::validation::Error::PatternError(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::RegularExpressionErrorParams::new(
                            #field_ident,
                            pattern,
                        ),
                        #message
                    )
                ));
        }
    )
}

fn get_pattern_from_list(
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
