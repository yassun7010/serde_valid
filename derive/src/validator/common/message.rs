use crate::abort::{
    abort_duplicated_argument, abort_unexpected_list_argument, abort_unexpected_name_value_argument,
};
use crate::types::SingleIdentPath;
use crate::validator::common::check_lit;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_message_tokens(
    validation_label: &str,
    field_ident: &syn::Ident,
    _attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Option<TokenStream> {
    let mut message_fmt = None;
    for meta_item in meta_items {
        match meta_item {
            syn::NestedMeta::Meta(ref item) => match item {
                syn::Meta::List(meta_list) => update_message_fn_from_meta_list(
                    validation_label,
                    &mut message_fmt,
                    field_ident,
                    meta_list,
                ),
                syn::Meta::Path(_) => continue,
                syn::Meta::NameValue(_) => continue,
            },
            syn::NestedMeta::Lit(_) => continue,
        }
    }
    message_fmt
}

fn update_message_fn_from_meta_path(
    validation_label: &str,
    message_fn: &mut Option<TokenStream>,
    field_ident: &syn::Ident,
    path: &syn::Path,
    path_ident: &syn::Ident,
) {
    check_duplicated_message_fn_argument(
        validation_label,
        message_fn,
        field_ident,
        path.span(),
        path_ident,
    );
    *message_fn = Some(quote!(#path));
}

fn update_message_fn_from_meta_list(
    validation_label: &str,
    message_fn: &mut Option<TokenStream>,
    field_ident: &syn::Ident,
    syn::MetaList { path, nested, .. }: &syn::MetaList,
) {
    let ident = SingleIdentPath::new(&path).ident();

    match ident.to_string().as_ref() {
        "message_fn" => {
            return update_message_fn_from_nested_meta(
                validation_label,
                message_fn,
                field_ident,
                nested,
                ident,
            )
        }
        _ => {}
    }
}

fn update_message_fn_from_nested_meta(
    validation_label: &str,
    message_fn: &mut Option<TokenStream>,
    field_ident: &syn::Ident,
    nested: &syn::punctuated::Punctuated<syn::NestedMeta, syn::Token![,]>,
    path_ident: &syn::Ident,
) {
    for meta_item in nested {
        match meta_item {
            syn::NestedMeta::Meta(ref item) => match item {
                syn::Meta::Path(path) => {
                    update_message_fn_from_meta_path(
                        validation_label,
                        message_fn,
                        field_ident,
                        path,
                        path_ident,
                    );
                }
                syn::Meta::List(list) => {
                    abort_unexpected_list_argument(validation_label, field_ident, item.span(), list)
                }
                syn::Meta::NameValue(name_value) => abort_unexpected_name_value_argument(
                    validation_label,
                    field_ident,
                    item.span(),
                    name_value,
                ),
            },
            syn::NestedMeta::Lit(lit) => check_lit(validation_label, field_ident, lit.span(), lit),
        }
    }
}

fn check_duplicated_message_fn_argument(
    validation_label: &str,
    message_fn: &mut Option<TokenStream>,
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    path_ident: &syn::Ident,
) {
    if message_fn.is_some() {
        abort_duplicated_argument(validation_label, field_ident, span, path_ident)
    }
}
