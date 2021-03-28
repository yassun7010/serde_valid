use crate::abort::{
    abort_duplicated_lit_argument, abort_required_path_argument, abort_unexpected_list_argument,
    abort_unexpected_path_argument, abort_unknown_name_value_argument,
};
use crate::types::SingleIdentPath;
use crate::validator::common::{check_lit, get_integer};
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_length_validator_tokens(
    validation_label: &str,
    min_label: &str,
    max_label: &str,
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> (TokenStream, TokenStream) {
    let mut min_value = None;
    let mut max_value = None;
    for meta_item in meta_items {
        match meta_item {
            syn::NestedMeta::Meta(ref item) => match item {
                syn::Meta::NameValue(name_value) => update_limit_value(
                    validation_label,
                    min_label,
                    max_label,
                    field_ident,
                    name_value,
                    &mut min_value,
                    &mut max_value,
                ),
                syn::Meta::List(list) => {
                    abort_unexpected_list_argument(
                        validation_label,
                        field_ident,
                        item.span(),
                        list,
                        true,
                    );
                }
                syn::Meta::Path(path) => {
                    abort_unexpected_path_argument(validation_label, field_ident, item.span(), path)
                }
            },
            syn::NestedMeta::Lit(lit) => check_lit(validation_label, field_ident, lit.span(), lit),
        }
    }
    let min_tokens = get_limit_tokens(min_value);
    let max_tokens = get_limit_tokens(max_value);

    if min_tokens.to_string() == "None" && max_tokens.to_string() == "None" {
        abort_required_path_argument(
            validation_label,
            &[min_label, max_label],
            field_ident,
            attribute.span(),
        );
    }
    (min_tokens, max_tokens)
}

fn update_limit_value(
    validation_label: &str,
    min_label: &str,
    max_label: &str,
    field_ident: &syn::Ident,
    name_value: &syn::MetaNameValue,
    min_value: &mut Option<syn::LitInt>,
    max_value: &mut Option<syn::LitInt>,
) {
    let syn::MetaNameValue { path, lit, .. } = name_value;
    let path_ident = SingleIdentPath::new(path).ident();
    let path_str = path_ident.to_string();
    if path_str == min_label {
        update_limit_int(validation_label, min_value, field_ident, lit);
    } else if path_str == max_label {
        update_limit_int(validation_label, max_value, field_ident, lit);
    } else {
        abort_unknown_name_value_argument(
            validation_label,
            &path_str,
            &[min_label, max_label],
            field_ident,
            path.span(),
            name_value,
        );
    }
}

fn update_limit_int(
    validation_label: &str,
    target: &mut Option<syn::LitInt>,
    field_ident: &syn::Ident,
    lit: &syn::Lit,
) {
    if target.is_some() {
        abort_duplicated_lit_argument(validation_label, field_ident, lit.span());
    }
    *target = Some(get_integer(validation_label, field_ident, lit));
}

fn get_limit_tokens(limit: Option<syn::LitInt>) -> proc_macro2::TokenStream {
    match limit {
        Some(value) => quote!(Some(#value)),
        None => quote!(None),
    }
}
