use crate::abort::{
    abort_duplicated_argument, abort_unknown_list_argument, abort_unknown_name_value_argument,
};
use crate::types::{Field, SingleIdentPath};
use crate::validator::common::check_lit;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_message_tokens(
    validation_label: &str,
    field: &impl Field,
    _attribute: &syn::Attribute,
    validation_args: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Option<TokenStream> {
    let mut message_fmt = None;
    for validation_arg in validation_args {
        match validation_arg {
            syn::NestedMeta::Meta(ref meta) => match meta {
                syn::Meta::List(message_fn_list) => update_message_fn_from_meta_list(
                    validation_label,
                    &mut message_fmt,
                    field,
                    message_fn_list,
                ),
                syn::Meta::Path(_) => continue,
                syn::Meta::NameValue(message_fn_key_value) => {
                    update_message_fn_from_meta_name_value(
                        validation_label,
                        &mut message_fmt,
                        field,
                        message_fn_key_value,
                    )
                }
            },
            syn::NestedMeta::Lit(_) => continue,
        }
    }
    message_fmt
}

fn update_message_fn_from_meta_path(
    validation_label: &str,
    message_fn: &mut Option<TokenStream>,
    field: &impl Field,
    fn_name: &syn::Path,
    message_fn_ident: &syn::Ident,
) {
    check_duplicated_message_fn_argument(
        validation_label,
        message_fn,
        field,
        fn_name,
        message_fn_ident,
    );
    *message_fn = Some(quote!(#fn_name));
}

fn update_message_fn_from_meta_list(
    validation_label: &str,
    message_fn: &mut Option<TokenStream>,
    field: &impl Field,
    syn::MetaList {
        path: name,
        nested: message_fn_defines,
        ..
    }: &syn::MetaList,
) {
    let message_ident = SingleIdentPath::new(&name).ident();
    let message_label = message_ident.to_string();

    match message_label.as_ref() {
        "message_fn" => update_message_fn_from_nested_meta(
            validation_label,
            message_fn,
            field,
            message_fn_defines,
            message_ident,
        ),
        _ => {}
    }
}

fn update_message_fn_from_meta_name_value(
    validation_label: &str,
    message_fn: &mut Option<TokenStream>,
    field: &impl Field,
    syn::MetaNameValue {
        path: name,
        lit: message,
        ..
    }: &syn::MetaNameValue,
) {
    let message_ident = SingleIdentPath::new(&name).ident();
    let message_label = message_ident.to_string();

    match message_label.as_ref() {
        "message" => {
            return update_message_fn_from_lit(validation_label, message_fn, field, message)
        }
        _ => {}
    }
}

fn update_message_fn_from_nested_meta(
    validation_label: &str,
    message_fn: &mut Option<TokenStream>,
    field: &impl Field,
    message_fn_defines: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
    message_fn_ident: &syn::Ident,
) {
    for message_fn_define in message_fn_defines {
        match message_fn_define {
            syn::NestedMeta::Meta(ref meta) => match meta {
                syn::Meta::Path(fn_name) => {
                    update_message_fn_from_meta_path(
                        validation_label,
                        message_fn,
                        field,
                        fn_name,
                        message_fn_ident,
                    );
                }
                syn::Meta::List(fn_define) => {
                    abort_unknown_list_argument(validation_label, field, meta.span(), fn_define)
                }
                syn::Meta::NameValue(name_value) => abort_unknown_name_value_argument(
                    validation_label,
                    field,
                    meta.span(),
                    name_value,
                ),
            },
            syn::NestedMeta::Lit(lit) => check_lit(validation_label, field, lit.span(), lit),
        }
    }
}

fn update_message_fn_from_lit(
    validation_label: &str,
    message_fn: &mut Option<TokenStream>,
    field: &impl Field,
    lit: &syn::Lit,
) {
    match lit {
        syn::Lit::Str(message) => *message_fn = Some(quote!(|_| { #message.to_string() })),
        _ => check_lit(validation_label, field, lit.span(), lit),
    }
}

fn check_duplicated_message_fn_argument(
    validation_label: &str,
    message_fn: &mut Option<TokenStream>,
    field: &impl Field,
    fn_name: &syn::Path,
    message_fn_ident: &syn::Ident,
) {
    if message_fn.is_some() {
        abort_duplicated_argument(validation_label, field, fn_name.span(), message_fn_ident)
    }
}
