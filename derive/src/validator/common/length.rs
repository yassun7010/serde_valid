use crate::abort::{
    abort_duplicated_lit_argument, abort_required_path_argument,
    abort_unexpected_name_value_argument, abort_unknown_list_argument, abort_unknown_path_argument,
};
use crate::types::Field;
use crate::types::SingleIdentPath;
use crate::validator::common::check::{
    check_common_meta_list_argument, check_common_meta_name_value_argument,
};
use crate::validator::common::{check_lit, get_integer};
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_length_validator_tokens<F: Field>(
    validation_label: &str,
    min_label: &str,
    max_label: &str,
    field: &F,
    attribute: &syn::Attribute,
    validation_args: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> (TokenStream, TokenStream) {
    let mut min_value = None;
    let mut max_value = None;
    for validation_arg in validation_args {
        match validation_arg {
            syn::NestedMeta::Meta(ref arg) => match arg {
                syn::Meta::NameValue(limit_name_value) => update_limit_value(
                    validation_label,
                    min_label,
                    max_label,
                    field,
                    limit_name_value,
                    &mut min_value,
                    &mut max_value,
                ),
                syn::Meta::List(list) => {
                    if !check_common_meta_list_argument(list) {
                        abort_unknown_list_argument(validation_label, field, arg.span(), list);
                    }
                }
                syn::Meta::Path(path) => {
                    abort_unknown_path_argument(validation_label, field, arg.span(), path)
                }
            },
            syn::NestedMeta::Lit(lit) => check_lit(validation_label, field, lit.span(), lit),
        }
    }
    let min_tokens = get_limit_tokens(min_value);
    let max_tokens = get_limit_tokens(max_value);

    if min_tokens.to_string() == "None" && max_tokens.to_string() == "None" {
        abort_required_path_argument(
            validation_label,
            &[min_label, max_label],
            field,
            attribute.span(),
        );
    }
    (min_tokens, max_tokens)
}

fn update_limit_value<F: Field>(
    validation_label: &str,
    min_label: &str,
    max_label: &str,
    field: &F,
    limit_name_value: &syn::MetaNameValue,
    min_value: &mut Option<syn::LitInt>,
    max_value: &mut Option<syn::LitInt>,
) {
    let syn::MetaNameValue {
        path: limit_name,
        lit: limit_value,
        ..
    } = limit_name_value;
    let limit_name_ident = SingleIdentPath::new(limit_name).ident();
    let limit_name_label = limit_name_ident.to_string();
    if limit_name_label == min_label {
        update_limit_int(validation_label, min_value, field, limit_value);
    } else if limit_name_label == max_label {
        update_limit_int(validation_label, max_value, field, limit_value);
    } else {
        if !check_common_meta_name_value_argument(limit_name_value) {
            abort_unexpected_name_value_argument(
                validation_label,
                &limit_name_label,
                &[min_label, max_label],
                field,
                limit_name.span(),
                limit_name_value,
            );
        }
    }
}

fn update_limit_int<F: Field>(
    validation_label: &str,
    target: &mut Option<syn::LitInt>,
    field: &F,
    lit: &syn::Lit,
) {
    if target.is_some() {
        abort_duplicated_lit_argument(validation_label, field, lit.span());
    }
    *target = Some(get_integer(validation_label, field, lit));
}

fn get_limit_tokens(limit: Option<syn::LitInt>) -> proc_macro2::TokenStream {
    match limit {
        Some(value) => quote!(Some(#value)),
        None => quote!(None),
    }
}
