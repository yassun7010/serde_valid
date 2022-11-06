use crate::types::{CommaSeparatedNestedMetas, SingleIdentPath};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

use super::{get_str, MetaListMessage, MetaNameValueMessage, MetaPathMessage};

#[derive(Debug, Default)]
pub struct CustomMessageToken {
    pub message_fn: Option<TokenStream>,
    #[cfg(feature = "fluent")]
    pub fluent_message: Option<TokenStream>,
}

impl CustomMessageToken {
    pub fn new_message_fn(message_fn: TokenStream) -> Self {
        Self {
            message_fn: Some(message_fn),
            #[cfg(feature = "fluent")]
            fluent_message: None,
        }
    }

    #[cfg(feature = "fluent")]
    pub fn new_fluent_message(fluent_message: TokenStream) -> Self {
        Self {
            message_fn: None,
            fluent_message: Some(fluent_message),
        }
    }
}

pub fn extract_custom_message_tokens(
    nested_meta: &syn::NestedMeta,
) -> Result<CustomMessageToken, crate::Errors> {
    match nested_meta {
        syn::NestedMeta::Meta(meta) => match meta {
            syn::Meta::List(message_fn_list) => {
                extract_custom_message_tokens_from_meta_list(message_fn_list)
            }
            syn::Meta::NameValue(name_value) => {
                extract_custom_message_tokens_from_name_value(name_value)
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

fn extract_custom_message_tokens_from_meta_list(
    syn::MetaList {
        path,
        nested: message_fn_define,
        ..
    }: &syn::MetaList,
) -> Result<CustomMessageToken, crate::Errors> {
    let path_ident = SingleIdentPath::new(path).ident();
    let path_label = path_ident.to_string();

    match MetaListMessage::from_str(&path_label) {
        Ok(MetaListMessage::MessageFn) => get_message_fn_from_nested_meta(path, message_fn_define)
            .map(CustomMessageToken::new_message_fn),
        #[cfg(feature = "fluent")]
        Ok(ref message_type @ (MetaListMessage::I18n | MetaListMessage::Fluent)) => {
            get_fluent_message_from_nested_meta(message_type, path, message_fn_define)
                .map(CustomMessageToken::new_fluent_message)
        }
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

fn extract_custom_message_tokens_from_name_value(
    syn::MetaNameValue { path, lit, .. }: &syn::MetaNameValue,
) -> Result<CustomMessageToken, crate::Errors> {
    let path_ident = SingleIdentPath::new(path).ident();
    let path_label = path_ident.to_string();

    match MetaNameValueMessage::from_str(&path_label) {
        Ok(MetaNameValueMessage::Message) => {
            get_message_fn_from_lit(lit).map(CustomMessageToken::new_message_fn)
        }
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
                syn::NestedMeta::Meta(syn::Meta::Path(fn_name)) => Some(quote!(#fn_name)),
                _ => None,
            };
            fn_name.ok_or_else(|| vec![crate::Error::message_fn_allow_name_path(&fn_define[0])])
        }
        _ => Err(fn_define
            .iter()
            .skip(1)
            .map(crate::Error::message_fn_tail_error)
            .collect()),
    }
}

fn get_message_fn_from_lit(lit: &syn::Lit) -> Result<TokenStream, crate::Errors> {
    get_str(lit).map(|lit_str| quote!(|_| { #lit_str.to_string() }))
}

#[cfg(feature = "fluent")]
fn get_fluent_message_from_nested_meta(
    message_type: &MetaListMessage,
    path: &syn::Path,
    fn_define: &CommaSeparatedNestedMetas,
) -> Result<TokenStream, crate::Errors> {
    match fn_define.len() {
        0 => Err(vec![crate::Error::fluent_need_item(message_type, path)]),
        1 => match &fn_define[0] {
            syn::NestedMeta::Lit(syn::Lit::Str(id)) => Ok(quote!(
                ::serde_valid::fluent::Message{
                    id: #id,
                    args: vec![]
                }
            )),
            _ => Err(vec![crate::Error::fluent_allow_key(
                message_type,
                &fn_define[0],
            )]),
        },
        _ => Err(fn_define
            .iter()
            .skip(1)
            .map(|args| crate::Error::fluent_allow_args(message_type, args))
            .collect()),
    }
}
