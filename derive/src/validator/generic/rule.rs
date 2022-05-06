use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_quote;

pub fn collect_rules(attributes: &Vec<syn::Attribute>) -> Result<Vec<TokenStream>, crate::Errors> {
    let mut errors: crate::Errors = vec![];

    let rules = attributes
        .iter()
        .filter(|attribute| attribute.path == parse_quote!(rule))
        .filter_map(|attribute| match extra_rule(attribute) {
            Ok(validator) => Some(validator),
            Err(error) => {
                errors.extend(error);
                None
            }
        })
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(rules)
}

fn extra_rule(attribute: &syn::Attribute) -> Result<TokenStream, crate::Errors> {
    match attribute.parse_meta() {
        Ok(syn::Meta::List(list)) => extra_meta_rule(&list),
        Ok(syn::Meta::NameValue(name_value)) => {
            Err(vec![crate::Error::meta_name_value_not_support(&name_value)])
        }
        Ok(syn::Meta::Path(path)) => Err(vec![crate::Error::rule_need_arguments(&path)]),
        Err(error) => return Err(vec![crate::Error::attribute_parse_error(attribute, &error)]),
    }
}

fn extra_meta_rule(
    syn::MetaList {
        path: rule_fn_name,
        ref nested,
        ..
    }: &syn::MetaList,
) -> Result<TokenStream, crate::Errors> {
    let mut errors = vec![];

    let field = if let Some(arg) = nested.first() {
        match arg {
            syn::NestedMeta::Meta(meta) => {
                match meta {
                    syn::Meta::Path(_) => (),
                    _ => errors.push(crate::Error::rule_required_first_argument_path(arg)),
                };
            }
            syn::NestedMeta::Lit(_) => {
                errors.push(crate::Error::rule_required_first_argument_path(arg));
            }
        }
        arg.to_token_stream()
    } else {
        Err(vec![crate::Error::rule_need_arguments(rule_fn_name)])?
    };

    let rule_fn_args = nested
        .iter()
        .filter_map(|arg| match arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(path) => Some(path.to_token_stream()),
                syn::Meta::List(list) => {
                    errors.push(crate::Error::meta_list_not_support(list));
                    None
                }
                syn::Meta::NameValue(name_value) => {
                    errors.push(crate::Error::meta_name_value_not_support(name_value));
                    None
                }
            },
            syn::NestedMeta::Lit(lit) => Some(lit.to_token_stream()),
        })
        .collect::<syn::punctuated::Punctuated<TokenStream, syn::token::Comma>>();

    if errors.len() > 0 {
        return Err(errors);
    }

    Ok(quote!(
        if let Err(__error) = #rule_fn_name(#rule_fn_args) {
            __errors
                .entry(#field)
                .or_default()
                .push(__error);
        };
    ))
}
