use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_quote;
use syn::spanned::Spanned;

use crate::types::CommaSeparatedTokenStreams;

pub fn collect_rules_from_unnamed_struct(
    attributes: &Vec<syn::Attribute>,
) -> Result<(HashSet<syn::Ident>, TokenStream), crate::Errors> {
    let mut errors: crate::Errors = vec![];

    let mut rule_fields = HashSet::new();
    let rules = attributes
        .iter()
        .filter(|attribute| attribute.path == parse_quote!(rule))
        .filter_map(|attribute| match attribute.parse_meta() {
            Ok(syn::Meta::List(list)) => match collect_rule(&list) {
                Ok((field_ident, stream)) => {
                    rule_fields.insert(field_ident);
                    Some(stream)
                }
                Err(rule_errors) => {
                    errors.extend(rule_errors);
                    None
                }
            },
            Ok(_) => {
                errors.push(crate::Error::rule_need_function(attribute.path.span()));
                None
            }
            Err(error) => {
                errors.push(crate::Error::rule_attribute_parse_error(attribute, &error));
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
    syn::MetaList {
        path, ref nested, ..
    }: &syn::MetaList,
) -> Result<(syn::Ident, TokenStream), crate::Errors> {
    let mut errors: crate::Errors = vec![];

    match nested.len() {
        0 => Err(vec![crate::Error::rule_need_function(path.span())])?,
        2.. => nested.iter().skip(1).for_each(|nested_meta| {
            errors.push(crate::Error::rule_allow_single_function(nested_meta.span()))
        }),
        _ => {}
    }

    let rule = match &nested[0] {
        syn::NestedMeta::Meta(meta) => match meta {
            syn::Meta::List(list) => extract_rule_from_meta_list(&list),
            syn::Meta::NameValue(name_value) => {
                Err(vec![crate::Error::meta_name_value_not_support(&name_value)])
            }
            syn::Meta::Path(path) => Err(vec![crate::Error::meta_path_not_support(&path)]),
        },
        syn::NestedMeta::Lit(lit) => Err(vec![crate::Error::literal_not_support(&lit)]),
    };

    match rule {
        Ok(_) => {
            if errors.is_empty() {
                rule
            } else {
                Err(errors)
            }
        }
        Err(rule_errors) => Err(errors.into_iter().chain(rule_errors.into_iter()).collect()),
    }
}

fn extract_rule_from_meta_list(
    syn::MetaList {
        path: rule_fn_name,
        ref nested,
        ..
    }: &syn::MetaList,
) -> Result<(syn::Ident, TokenStream), crate::Errors> {
    let mut errors = vec![];

    if nested.is_empty() {
        errors.push(crate::Error::rule_need_arguments(rule_fn_name));
    }

    let mut first_arg = None;
    let rule_fn_args = nested
        .iter()
        .filter_map(|nested_meta| {
            let arg = match nested_meta {
                syn::NestedMeta::Lit(lit) => match lit {
                    syn::Lit::Int(int) => {
                        let index = syn::Ident::new(&format!("__{}", int), int.span());
                        first_arg = Some(index.clone());
                        Some(quote!(#index))
                    }
                    _ => None,
                },
                syn::NestedMeta::Meta(_) => None,
            };
            if arg.is_none() {
                errors.push(crate::Error::rule_allow_index_arguments(
                    rule_fn_name,
                    nested_meta,
                ));
            }
            arg
        })
        .collect::<CommaSeparatedTokenStreams>();

    match first_arg {
        Some(field_ident) => {
            if errors.len() > 0 {
                return Err(errors);
            }

            let field_name = nested[0].to_token_stream().to_string();
            Ok((
                field_ident,
                quote!(
                    if let Err(__error) = #rule_fn_name(#rule_fn_args) {
                        __errors
                            .entry(#field_name)
                            .or_default()
                            .push(__error);
                    };
                ),
            ))
        }
        None => Err(errors),
    }
}
