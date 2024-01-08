use crate::types::{CommaSeparatedNestedMetas, NestedMeta, SingleIdentPath};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

use super::{get_str, MetaListCustomMessage, MetaNameValueCustomMessage, MetaPathCustomMessage};

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

    pub fn into_token(self) -> TokenStream {
        let message_fn = self.message_fn.unwrap_or(quote!(
            ::serde_valid::validation::ToDefaultMessage::to_default_message
        ));

        #[cfg(feature = "fluent")]
        let fluent_message = quote!(fluent_message: None,);
        #[cfg(not(feature = "fluent"))]
        let fluent_message = quote!();

        quote!(
            ::serde_valid::validation::CustomMessage{
                message_fn: #message_fn,
                #fluent_message
            }
        )
    }
}

pub fn extract_custom_message_tokens(
    meta: &syn::Meta,
) -> Result<CustomMessageToken, crate::Errors> {
    let custom_message_path = match meta {
        syn::Meta::Path(path) => path,
        syn::Meta::List(list) => &list.path,
        syn::Meta::NameValue(name_value) => &name_value.path,
    };
    let custom_message_name = SingleIdentPath::new(custom_message_path)
        .ident()
        .to_string();

    match (
        MetaPathCustomMessage::from_str(&custom_message_name),
        MetaListCustomMessage::from_str(&custom_message_name),
        MetaNameValueCustomMessage::from_str(&custom_message_name),
        meta,
    ) {
        (Ok(_), _, _, syn::Meta::Path(_)) => {
            unreachable!()
        }
        (_, Ok(custom_message_type), _, syn::Meta::List(custom_message)) => {
            extract_custom_message_tokens_from_meta_list(&custom_message_type, custom_message)
        }
        (_, _, Ok(custom_message_type), syn::Meta::NameValue(custom_message)) => {
            extract_custom_message_tokens_from_name_value(&custom_message_type, custom_message)
        }
        (Ok(_), _, _, _) => Err(vec![crate::Error::meta_path_custom_message_need_value(
            custom_message_path,
            &custom_message_name,
        )]),
        (_, Ok(_), _, _) => Err(vec![crate::Error::meta_list_custom_message_need_value(
            custom_message_path,
            &custom_message_name,
        )]),
        (_, _, Ok(_), _) => Err(vec![
            crate::Error::meta_name_value_custom_message_need_value(
                custom_message_path,
                &custom_message_name,
            ),
        ]),
        _ => Err(vec![crate::Error::unknown_custom_message_type(
            custom_message_path,
            &custom_message_name,
        )]),
    }
}

fn extract_custom_message_tokens_from_meta_list(
    custom_message_type: &MetaListCustomMessage,
    meta_list: &syn::MetaList,
) -> Result<CustomMessageToken, crate::Errors> {
    let path = &meta_list.path;
    let path_ident = SingleIdentPath::new(path).ident();
    let message_fn_define = meta_list
        .parse_args_with(CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::custom_message_parse_error(path_ident, &error)])?;

    match custom_message_type {
        MetaListCustomMessage::MessageFn => {
            get_message_fn_from_nested_meta(path, &message_fn_define)
                .map(CustomMessageToken::new_message_fn)
        }
        #[cfg(feature = "fluent")]
        message_type @ (MetaListCustomMessage::I18n | MetaListCustomMessage::Fluent) => {
            get_fluent_message_from_nested_meta(message_type, path, &message_fn_define)
                .map(CustomMessageToken::new_fluent_message)
        }
    }
}

fn extract_custom_message_tokens_from_name_value(
    custom_message_type: &MetaNameValueCustomMessage,
    name_value: &syn::MetaNameValue,
) -> Result<CustomMessageToken, crate::Errors> {
    match custom_message_type {
        MetaNameValueCustomMessage::Message => {
            get_message_from_expr(&name_value.value).map(CustomMessageToken::new_message_fn)
        }
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
                NestedMeta::Meta(syn::Meta::Path(fn_name)) => Some(quote!(#fn_name)),
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

fn get_message_from_expr(expr: &syn::Expr) -> Result<TokenStream, crate::Errors> {
    match expr {
        syn::Expr::Lit(lit) => get_message_from_lit(&lit.lit),
        _ => Err(vec![crate::Error::literal_only(expr)]),
    }
}

fn get_message_from_lit(lit: &syn::Lit) -> Result<TokenStream, crate::Errors> {
    get_str(lit).map(|lit_str| quote!(|_| { #lit_str.to_string() }))
}

#[cfg(feature = "fluent")]
fn get_fluent_message_from_nested_meta(
    message_type: &MetaListCustomMessage,
    path: &syn::Path,
    fn_define: &CommaSeparatedNestedMetas,
) -> Result<TokenStream, crate::Errors> {
    match fn_define.len() {
        0 => Err(vec![crate::Error::fluent_need_item(message_type, path)]),
        1 => match &fn_define[0] {
            NestedMeta::Lit(syn::Lit::Str(id)) => Ok(quote!(
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
