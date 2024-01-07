use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

use crate::types::{CommaSeparatedNestedMetas, CommaSeparatedTokenStreams, NestedMeta};

pub fn collect_rules_from_unnamed_struct(
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
                errors.push(crate::Error::rule_allow_function_call_or_closure(
                    attribute.meta.path(),
                ));
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
        .parse_args_with(CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::rule_args_parse_error(metalist, &error)])?;

    match nested.len() {
        0 => Err(vec![crate::Error::rule_allow_function_call_or_closure(
            metalist.path.span(),
        )])?,
        2.. => nested.iter().skip(1).for_each(|nested_meta| {
            errors.push(crate::Error::rule_allow_single_function(nested_meta))
        }),
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
    metalist: &syn::MetaList,
) -> Result<(HashSet<syn::Ident>, TokenStream), crate::Errors> {
    let mut errors = vec![];

    let rule_fn_name = &metalist.path;
    let nested = metalist
        .parse_args_with(CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::rule_args_parse_error(metalist, &error)])?;
    if nested.is_empty() {
        errors.push(crate::Error::rule_need_arguments(rule_fn_name));
    }

    let mut arg_idents = HashSet::new();
    let rule_fn_args = nested
        .iter()
        .filter_map(|nested_meta| match nested_meta {
            NestedMeta::Lit(syn::Lit::Int(int)) => {
                let index = syn::Ident::new(&format!("__{}", int), int.span());
                arg_idents.insert(index.clone());
                Some(quote!(#index))
            }
            _ => {
                errors.push(crate::Error::rule_args_allow_field_index(
                    rule_fn_name,
                    nested_meta,
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
                __rule_vec_errors
                    .push(__error);
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
        .filter_map(|input| match input {
            syn::Pat::Ident(syn::PatIdent { ident, .. }) => {
                let index = syn::Ident::new(&format!("_{}", ident), ident.span());
                arg_idents.insert(index.clone());

                Some(quote!(#index))
            }
            _ => {
                errors.push(crate::Error::rule_unnamed_clousure_input(input));

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
