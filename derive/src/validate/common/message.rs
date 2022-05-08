use crate::types::{CommaSeparatedNestedMetas, SingleIdentPath};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

use super::{get_str, MetaListMessage, MetaNameValueMessage, MetaPathMessage};

pub fn extract_message_fn_tokens(
    nested_meta: &syn::NestedMeta,
) -> Result<TokenStream, crate::Errors> {
    match nested_meta {
        syn::NestedMeta::Meta(meta) => match meta {
            syn::Meta::List(message_fn_list) => {
                extract_message_fn_tokens_from_meta_list(message_fn_list)
            }
            syn::Meta::NameValue(name_value) => {
                extract_message_fn_tokens_from_name_value(name_value)
            }
            syn::Meta::Path(path) => {
                let path_label = SingleIdentPath::new(path).ident().to_string();
                if MetaNameValueMessage::from_str(&path_label).is_ok() {
                    Err(crate::Error::validate_meta_name_value_need_value(
                        path,
                        &path_label,
                    ))
                } else if MetaListMessage::from_str(&path_label).is_ok() {
                    Err(crate::Error::validate_meta_list_need_value(
                        path,
                        &path_label,
                    ))
                } else {
                    Err(crate::Error::validate_unknown_type(
                        path,
                        &path_label,
                        &(MetaNameValueMessage::iter().map(|x| x.name()))
                            .chain(MetaListMessage::iter().map(|x| x.name()))
                            .chain(MetaPathMessage::iter().map(|x| x.name()))
                            .collect::<Vec<_>>(),
                    ))
                }
            }
            .map_err(|error| vec![error]),
        },
        syn::NestedMeta::Lit(lit) => Err(vec![crate::Error::literal_not_support(lit)]),
    }
}

fn extract_message_fn_tokens_from_meta_list(
    syn::MetaList {
        path,
        nested: message_fn_define,
        ..
    }: &syn::MetaList,
) -> Result<TokenStream, crate::Errors> {
    let path_ident = SingleIdentPath::new(&path).ident();
    let path_label = path_ident.to_string();

    match MetaListMessage::from_str(&path_label) {
        Ok(MetaListMessage::MessageFn) => get_message_fn_from_nested_meta(path, message_fn_define),
        Err(unknown) => {
            let error = if MetaNameValueMessage::from_str(&path_label).is_ok() {
                crate::Error::validate_meta_list_need_value(path, &path_label)
            } else if MetaPathMessage::from_str(&path_label).is_ok() {
                crate::Error::validate_meta_path_need_value(path, &path_label)
            } else {
                crate::Error::validate_unknown_type(
                    path,
                    &unknown,
                    &MetaListMessage::iter()
                        .map(|x| x.name())
                        .collect::<Vec<_>>(),
                )
            };
            Err(vec![error])
        }
    }
}

fn extract_message_fn_tokens_from_name_value(
    syn::MetaNameValue { path, lit, .. }: &syn::MetaNameValue,
) -> Result<TokenStream, crate::Errors> {
    let path_ident = SingleIdentPath::new(&path).ident();
    let path_label = path_ident.to_string();

    match MetaNameValueMessage::from_str(&path_label) {
        Ok(MetaNameValueMessage::Message) => get_message_fn_from_lit(lit),
        Err(unknown) => if MetaListMessage::from_str(&path_label).is_ok() {
            Err(crate::Error::validate_meta_list_need_value(
                path,
                &path_label,
            ))
        } else if MetaPathMessage::from_str(&path_label).is_ok() {
            Err(crate::Error::validate_meta_path_need_value(
                path,
                &path_label,
            ))
        } else {
            Err(crate::Error::validate_unknown_type(
                path,
                &unknown,
                &MetaNameValueMessage::iter()
                    .map(|x| x.name())
                    .collect::<Vec<_>>(),
            ))
        }
        .map_err(|error| vec![error]),
    }
}

fn get_message_fn_from_nested_meta(
    path: &syn::Path,
    fn_define: &CommaSeparatedNestedMetas,
) -> Result<TokenStream, crate::Errors> {
    match fn_define.len() {
        0 => Err(vec![crate::Error::message_fn_need_item(path)]),
        1 => {
            let fn_name = match &fn_define[0] {
                syn::NestedMeta::Meta(ref meta) => match meta {
                    syn::Meta::Path(fn_name) => Some(quote!(#fn_name)),
                    _ => None,
                },
                _ => None,
            };
            fn_name.ok_or(vec![crate::Error::message_fn_allow_name_path(
                &fn_define[0],
            )])
        }
        _ => Err(fn_define
            .iter()
            .skip(1)
            .map(|arg| crate::Error::message_fn_tail_error(arg))
            .collect()),
    }
}

fn get_message_fn_from_lit(lit: &syn::Lit) -> Result<TokenStream, crate::Errors> {
    get_str(lit).map(|lit_str| quote!(|_| { #lit_str.to_string() }))
}
