mod meta;
mod number;

use crate::abort::abort_invalid_attribute_on_field;
use crate::helper::{NamedField, NamedFieldBuf};
use meta::extract_meta_validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use std::iter::FromIterator;
use syn::parse_quote;
use syn::spanned::Spanned;

pub enum Validator {
    Normal(TokenStream),
    #[allow(dead_code)]
    Option(TokenStream),
}

pub struct FieldValidators {
    field: NamedFieldBuf,
    validators: Vec<TokenStream>,
    optional_validators: Vec<TokenStream>,
}

impl FieldValidators {
    pub fn new(field: syn::Field) -> Self {
        Self {
            field: NamedFieldBuf::new(field),
            validators: vec![],
            optional_validators: vec![],
        }
    }

    pub fn push(&mut self, validator: Validator) {
        match validator {
            Validator::Normal(token) => self.validators.push(token),
            Validator::Option(token) => self.optional_validators.push(token),
        }
    }

    pub fn to_token(&self) -> TokenStream {
        let ident = self.field.ident();

        let normal_tokens = if !self.validators.is_empty() {
            let validators = TokenStream::from_iter(self.validators.clone());
            quote! (#validators)
        } else {
            quote! {}
        };

        let optional_tokens = if !self.optional_validators.is_empty() {
            let option_validators =
                TokenStream::from_iter(self.optional_validators.clone().into_iter());
            quote!(
                if let Some(v) = #ident {
                    #option_validators
                }
            )
        } else {
            quote!()
        };

        quote!(
            #normal_tokens
            #optional_tokens
        )
    }
}

/// Find the types (as string) for each field of the struct
/// Needed for the `must_match` filter
pub fn collect_validators(fields: &syn::Fields) -> Vec<FieldValidators> {
    let mut struct_validators = vec![];
    for field in fields {
        let mut field_validators = FieldValidators::new(field.clone());
        let named_field = NamedField::new(field);
        let field_ident = named_field.ident();
        for attribute in named_field.attributes() {
            if attribute.path != parse_quote!(validate) {
                continue;
            }
            let validator = extract_meta_validator(&named_field, attribute);
            match validator {
                Some(validator) => field_validators.push(validator),
                None => abort_invalid_attribute_on_field(
                    &field_ident,
                    attribute.span(),
                    "it needs at least one validator",
                ),
            }
        }
        struct_validators.push(field_validators)
    }

    struct_validators
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
