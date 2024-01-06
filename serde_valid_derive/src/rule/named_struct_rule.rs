use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

use crate::types::{CommaSeparatedMetas, CommaSeparatedTokenStreams};

pub fn collect_rules_from_named_struct(
    attributes: &[syn::Attribute],
) -> Result<(HashSet<syn::Ident>, TokenStream), crate::Errors> {
    let mut errors = vec![];

    let mut rule_fields = HashSet::new();
    let rules = attributes
        .iter()
        .filter(|attribute| attribute.path().is_ident("rule"))
        .filter_map(|attribute| match &attribute.meta {
            syn::Meta::List(list) => match collect_rule(list) {
                Ok((field_ident, stream)) => {
                    rule_fields.extend(field_ident);
                    Some(stream)
                }
                Err(rule_errors) => {
                    errors.extend(rule_errors);
                    None
                }
            },
            _ => {
                errors.push(crate::Error::rule_need_function(attribute.meta.path()));
                None
            }
        })
        .collect::<Vec<_>>();

    if errors.is_empty() {
        Ok((rule_fields, TokenStream::from_iter(rules)))
    } else {
        Err(errors)
    }
}

fn collect_rule(
    metalist: &syn::MetaList,
) -> Result<(HashSet<syn::Ident>, TokenStream), crate::Errors> {
    let mut errors = vec![];

    let nested = metalist
        .parse_args_with(CommaSeparatedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::rule_args_parse_error(metalist, &error)])?;

    match nested.len() {
        0 => Err(vec![crate::Error::rule_need_function(&metalist.path)])?,
        2.. => nested
            .iter()
            .skip(1)
            .for_each(|error| errors.push(crate::Error::rule_allow_single_function(error))),
        _ => {}
    }

    let rule = match &nested[0] {
        syn::Meta::List(list) => extract_rule_from_meta_list(list),
        syn::Meta::NameValue(name_value) => {
            Err(vec![crate::Error::meta_name_value_not_support(name_value)])
        }
        syn::Meta::Path(path) => Err(vec![crate::Error::meta_path_not_support(path)]),
    };

    match rule {
        Ok(_) => {
            if errors.is_empty() {
                rule
            } else {
                Err(errors)
            }
        }
        Err(rule_errors) => Err(errors.into_iter().chain(rule_errors).collect()),
    }
}

fn extract_rule_from_meta_list(
    metalist: &syn::MetaList,
) -> Result<(HashSet<syn::Ident>, TokenStream), crate::Errors> {
    let mut errors = vec![];

    let rule_fn_name = &metalist.path;
    let nested = metalist
        .parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated)
        .map_err(|error| vec![crate::Error::rule_args_parse_error(metalist, &error)])?;

    if nested.is_empty() {
        errors.push(crate::Error::rule_need_arguments(rule_fn_name));
    }

    let mut arg_idents = HashSet::new();
    let rule_fn_args = nested
        .iter()
        .filter_map(|path| {
            let arg = {
                let ident = path.to_token_stream().to_string();
                if ident == "r#type" {
                    arg_idents.insert(syn::Ident::new_raw("type", path.span()));
                    Some(quote!(r#type))
                } else {
                    arg_idents.insert(syn::Ident::new(&ident, path.span()));
                    Some(quote!(#path))
                }
            };
            if arg.is_none() {
                errors.push(crate::Error::rule_allow_path_arguments(
                    rule_fn_name,
                    &nested,
                ));
            }
            arg
        })
        .collect::<CommaSeparatedTokenStreams>();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok((
        arg_idents,
        quote!(
            if let Err(__error) = #rule_fn_name(#rule_fn_args) {
                __rule_vec_errors.push(__error);
            };
        ),
    ))
}
