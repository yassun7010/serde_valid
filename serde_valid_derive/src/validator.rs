mod meta;
mod number;

use crate::abort::{abort_invalid_attribute_on_field, abort_unnamed_fields_struct};
use meta::extract_meta_validator;
use proc_macro_error::abort;
use quote::ToTokens;
use syn::parse_quote;
use syn::spanned::Spanned;

/// Find the types (as string) for each field of the struct
/// Needed for the `must_match` filter
pub fn collect_validators(fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    let mut validators = vec![];
    for field in fields {
        let field_ident = &field
            .ident
            .as_ref()
            .unwrap_or_else(|| abort_unnamed_fields_struct(field.span()));
        for attribute in &field.attrs {
            if attribute.path != parse_quote!(validate) {
                continue;
            }
            let validator = extract_meta_validator(field_ident, attribute);
            match validator {
                Some(validator) => validators.push(validator),
                None => abort_invalid_attribute_on_field(
                    &field_ident,
                    attribute.span(),
                    "it needs at least one validator",
                ),
            }
        }
    }

    validators
}

#[allow(dead_code)]
fn get_field_type(field_type: &syn::Type, field_ident: &syn::Ident) -> String {
    match field_type {
        syn::Type::Path(syn::TypePath { ref path, .. }) => path.to_token_stream().to_string(),
        syn::Type::Reference(syn::TypeReference {
            ref lifetime,
            ref elem,
            ..
        }) => {
            if lifetime.is_some() {
                format!("&{}", elem.to_token_stream())
            } else {
                elem.to_token_stream().to_string()
            }
        }
        _ => {
            abort!(
                field_type.span(),
                "Type `{}` of field `{}` not supported",
                field_type.to_token_stream(),
                field_ident
            )
        }
    }
}

#[allow(dead_code)]
fn find_original_field_name<'a>(meta_items: &[&'a syn::NestedMeta]) -> Option<&'a syn::LitStr> {
    for meta_item in meta_items {
        match **meta_item {
            syn::NestedMeta::Meta(ref item) => match *item {
                syn::Meta::Path(_) => continue,
                syn::Meta::NameValue(syn::MetaNameValue {
                    ref path, ref lit, ..
                }) => {
                    let ident = path.get_ident().unwrap();
                    if ident == "rename" {
                        if let syn::Lit::Str(lit_str) = lit {
                            return Some(lit_str);
                        }
                    }
                }
                syn::Meta::List(syn::MetaList { ref nested, .. }) => {
                    return find_original_field_name(&nested.iter().collect::<Vec<_>>());
                }
            },
            _ => unreachable!(),
        };
    }
    None
}
