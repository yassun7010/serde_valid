use super::struct_named_fields::collect_struct_named_fields_validators;
use super::struct_unnamed_fields::collect_struct_unnamed_fields_validators;
use crate::errors::{fields_errors_tokens, single_errors_tokens};
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

type Variants = syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>;

pub fn expand_enum_variants_validators(
    enum_ident: &syn::Ident,
    variants: &Variants,
) -> (TokenStream, TokenStream) {
    let mut enum_validator_tokens = vec![];
    let mut is_fields_errors = true;
    for variant in variants.iter() {
        let variant_tokens = match &variant.fields {
            syn::Fields::Named(fields_named) => {
                let variant_ident = &variant.ident;
                let fields_validators = collect_struct_named_fields_validators(fields_named);
                let mut fields_idents =
                    syn::punctuated::Punctuated::<TokenStream, syn::Token!(,)>::new();
                let fields_validators_tokens =
                    TokenStream::from_iter(fields_validators.iter().map(|validators| {
                        let field_ident = validators.ident();
                        if let Some(token) = validators.get_tokens() {
                            fields_idents.push(quote!(#field_ident));
                            quote!(#token)
                        } else {
                            fields_idents.push(quote!(#field_ident: _));
                            quote!()
                        }
                    }));
                quote!(
                    if let #enum_ident::#variant_ident{#fields_idents} = &self {
                        #fields_validators_tokens
                    }
                )
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                let variant_ident = &variant.ident;
                let fields_validators = collect_struct_unnamed_fields_validators(fields_unnamed);
                let mut fields_idents =
                    syn::punctuated::Punctuated::<TokenStream, syn::Token!(,)>::new();
                let fields_validators_tokens =
                    TokenStream::from_iter(fields_validators.iter().map(|validators| {
                        if let Some(token) = validators.get_tokens() {
                            let ident = validators.ident();
                            fields_idents.push(quote!(#ident));
                            quote!(#token)
                        } else {
                            fields_idents.push(quote!(_));
                            quote!()
                        }
                    }));
                is_fields_errors = fields_validators.len() != 1;
                quote!(
                    if let #enum_ident::#variant_ident(#fields_idents) = &self {
                        #fields_validators_tokens
                    }
                )
            }
            syn::Fields::Unit => quote!(),
        };
        enum_validator_tokens.push(variant_tokens);
    }
    let validators = TokenStream::from_iter(enum_validator_tokens);
    let errors = if is_fields_errors {
        fields_errors_tokens()
    } else {
        single_errors_tokens()
    };
    (validators, errors)
}
