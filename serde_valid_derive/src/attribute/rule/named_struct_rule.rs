use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

use crate::{
    types::{CommaSeparatedNestedMetas, CommaSeparatedTokenStreams},
    warning::Warning,
    warning::WithWarnings,
};

pub fn collect_rules_from_named_struct(
    ident: &syn::Ident,
    attributes: &[syn::Attribute],
) -> Result<(HashSet<syn::Ident>, WithWarnings<TokenStream>), crate::Errors> {
    let mut errors = vec![];

    let mut rule_fields = HashSet::new();
    let mut warnings = vec![];
    let rules = attributes
        .iter()
        .filter(|attribute| attribute.path().is_ident("rule"))
        .inspect(|attribute| {
            warnings.push(Warning::new_rule_deprecated(
                ident,
                attribute.bracket_token.span.span(),
            ));
        })
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
                errors.push(crate::Error::rule_allow_function_call_or_closure(
                    attribute.meta.path(),
                ));
                None
            }
        })
        .collect::<Vec<_>>();

    if errors.is_empty() {
        Ok((
            rule_fields,
            WithWarnings {
                data: TokenStream::from_iter(rules),
                warnings,
            },
        ))
    } else {
        Err(errors)
    }
}

fn collect_rule(
    meta_list: &syn::MetaList,
) -> Result<(HashSet<syn::Ident>, TokenStream), crate::Errors> {
    let mut errors = vec![];

    let nested = meta_list
        .parse_args_with(CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::rule_args_parse_error(meta_list, &error)])?;

    match nested.len() {
        0 => Err(vec![crate::Error::rule_allow_function_call_or_closure(
            meta_list,
        )])?,
        2.. => nested
            .iter()
            .skip(1)
            .for_each(|error| errors.push(crate::Error::rule_allow_single_function(error))),
        _ => {}
    }

    let rule = match &nested[0] {
        crate::types::NestedMeta::Meta(syn::Meta::List(list)) => extract_rule_from_meta_list(list),
        crate::types::NestedMeta::Closure(closure) => extract_rule_from_closure(closure),
        _ => Err(vec![crate::Error::rule_allow_function_call_or_closure(
            &nested[0],
        )]),
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
    meta_list: &syn::MetaList,
) -> Result<(HashSet<syn::Ident>, TokenStream), crate::Errors> {
    let mut errors = vec![];

    let rule_fn_name = &meta_list.path;
    let nested = meta_list
        .parse_args_with(crate::types::CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::rule_args_parse_error(meta_list, &error)])?;

    if nested.is_empty() {
        errors.push(crate::Error::rule_need_arguments(rule_fn_name));
    }

    let mut arg_idents = HashSet::new();
    let rule_fn_args = nested
        .iter()
        .filter_map(|nested| match nested {
            crate::types::NestedMeta::Meta(syn::Meta::Path(path)) => {
                let ident = path.to_token_stream().to_string();
                if ident == "r#type" {
                    arg_idents.insert(syn::Ident::new_raw("type", path.span()));
                    Some(quote!(r#type))
                } else {
                    arg_idents.insert(syn::Ident::new(&ident, path.span()));
                    Some(quote!(#path))
                }
            }
            _ => {
                errors.push(crate::Error::rule_args_allow_field_name(
                    rule_fn_name,
                    nested,
                ));
                None
            }
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

fn extract_rule_from_closure(
    closure: &syn::ExprClosure,
) -> Result<(HashSet<syn::Ident>, TokenStream), crate::Errors> {
    let mut errors = vec![];

    let mut arg_idents = HashSet::new();
    let rule_fn_args = closure
        .inputs
        .iter()
        .filter_map(|input| {
            if let syn::Pat::Ident(ident) = input {
                arg_idents.insert(ident.ident.clone());
                Some(quote!(#ident))
            } else {
                errors.push(crate::Error::rule_named_clousure_input(input));
                None
            }
        })
        .collect::<CommaSeparatedTokenStreams>();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok((
        arg_idents,
        quote!(
            if let Err(__error) = (#closure)(#rule_fn_args) {
                __rule_vec_errors.push(__error);
            };
        ),
    ))
}
