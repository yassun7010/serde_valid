use crate::attribute::{MetaListCustomMessage, MetaNameValueCustomMessage, MetaPathCustomMessage};
use crate::types::{CommaSeparatedNestedMetas, NestedMeta, SingleIdentPath};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;

use super::lit::get_str;
pub type MessageFormat = TokenStream;

pub fn default_message_format() -> MessageFormat {
    quote!(::serde_valid::validation::error::Format::Default)
}

pub fn extract_custom_message_format(meta: &syn::Meta) -> Result<MessageFormat, crate::Errors> {
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
        (_, Ok(custom_message_type), _, syn::Meta::List(custom_message)) => Ok(
            extract_custom_message_format_from_meta_list(&custom_message_type, custom_message)?,
        ),
        (_, _, Ok(custom_message_type), syn::Meta::NameValue(custom_message)) => Ok(
            extract_custom_message_format_from_name_value(&custom_message_type, custom_message)?,
        ),
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

fn extract_custom_message_format_from_meta_list(
    custom_message_type: &MetaListCustomMessage,
    meta_list: &syn::MetaList,
) -> Result<MessageFormat, crate::Errors> {
    let path = &meta_list.path;
    let path_ident = SingleIdentPath::new(path).ident();
    let message_fn_define = meta_list
        .parse_args_with(CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::custom_message_parse_error(path_ident, &error)])?;

    match custom_message_type {
        MetaListCustomMessage::MessageFn => get_message_fn(path, &message_fn_define),
        #[cfg(feature = "fluent")]
        message_type @ (MetaListCustomMessage::I18n | MetaListCustomMessage::Fluent) => {
            get_fluent_message(message_type, path, &message_fn_define)
        }
    }
}

fn extract_custom_message_format_from_name_value(
    custom_message_type: &MetaNameValueCustomMessage,
    name_value: &syn::MetaNameValue,
) -> Result<MessageFormat, crate::Errors> {
    match custom_message_type {
        MetaNameValueCustomMessage::Message => get_message(&name_value.value),
    }
}

fn get_message_fn(
    path: &syn::Path,
    fn_define: &CommaSeparatedNestedMetas,
) -> Result<TokenStream, crate::Errors> {
    let fn_name = match fn_define.len() {
        0 => Err(vec![crate::Error::message_fn_need_item(path)]),
        1 => match &fn_define[0] {
            NestedMeta::Meta(syn::Meta::Path(fn_name)) => Some(quote!(#fn_name)),
            _ => None,
        }
        .ok_or_else(|| vec![crate::Error::message_fn_allow_name_path(&fn_define[0])]),
        _ => Err(fn_define
            .iter()
            .skip(1)
            .map(crate::Error::message_fn_tail_error)
            .collect()),
    }?;

    Ok(quote!(::serde_valid::validation::error::Format::MessageFn(#fn_name)))
}

fn get_message(expr: &syn::Expr) -> Result<TokenStream, crate::Errors> {
    match expr {
        syn::Expr::Lit(lit) => {
            get_str(&lit.lit).map(|lit_str| quote!(::serde_valid::validation::error::Format::Message(#lit_str.to_string())))
        }
        _ => Err(vec![crate::Error::literal_only(expr)]),
    }
}

#[cfg(feature = "fluent")]
fn get_fluent_message(
    message_type: &MetaListCustomMessage,
    path: &syn::Path,
    fn_define: &CommaSeparatedNestedMetas,
) -> Result<MessageFormat, crate::Errors> {
    use quote::ToTokens;

    use crate::types::CommaSeparatedTokenStreams;

    match fn_define.len() {
        0 => Err(vec![crate::Error::fluent_need_item(message_type, path)]),
        1 => {
            let id = get_fluent_id(&fn_define[0])
                .ok_or_else(|| vec![crate::Error::fluent_allow_key(message_type, &fn_define[0])])?;

            Ok(quote!(
                ::serde_valid::validation::error::Format::Fluent(
                    ::serde_valid::fluent::Message{
                        id: #id,
                        args: vec![]
                    }
                )
            ))
        }
        _ => {
            let mut errors = vec![];
            let id = get_fluent_id(&fn_define[0])
                .ok_or_else(|| vec![crate::Error::fluent_allow_key(message_type, &fn_define[0])])?;

            let args = fn_define
                .iter()
                .skip(1)
                .filter_map(|arg| {
                    if let NestedMeta::Meta(syn::Meta::NameValue(name_value)) = arg {
                        let name = &name_value.path.to_token_stream().to_string();
                        if let syn::Expr::Lit(lit) = &name_value.value {
                            return Some(
                                quote!((#name, ::serde_valid::fluent::FluentValue::from(#lit))),
                            );
                        } else {
                            errors.push(crate::Error::fluent_allow_args(message_type, arg));
                        }
                    } else {
                        errors.push(crate::Error::fluent_allow_args(message_type, arg));
                    }
                    None
                })
                .collect::<CommaSeparatedTokenStreams>();
            if errors.is_empty() {
                Ok(quote!(
                    ::serde_valid::validation::error::Format::Fluent(
                        ::serde_valid::fluent::Message{
                            id: #id,
                            args: vec![#args]
                        }
                    )
                ))
            } else {
                Err(errors)
            }
        }
    }
}

fn get_fluent_id(nested_meta: &NestedMeta) -> Option<&syn::LitStr> {
    match nested_meta {
        NestedMeta::Lit(syn::Lit::Str(id)) => Some(id),
        _ => None,
    }
}
