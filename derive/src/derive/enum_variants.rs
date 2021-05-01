use super::struct_named_fields::collect_struct_named_fields_validators;
use super::struct_unnamed_fields::collect_struct_unnamed_fields_validators;
use crate::errors::{fields_errors_tokens, new_type_errors_tokens};
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

type Variants = syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>;

pub fn expand_enum_variants_validate(enum_ident: &syn::Ident, variants: &Variants) -> TokenStream {
    let mut enum_validator_tokens = vec![];
    for variant in variants.iter() {
        let variant_tokens = match &variant.fields {
            syn::Fields::Named(fields_named) => {
                let variant_ident = &variant.ident;
                let fields_validators = collect_struct_named_fields_validators(fields_named);
                let mut fields_idents =
                    syn::punctuated::Punctuated::<TokenStream, syn::token::Comma>::new();
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
                let errors = fields_errors_tokens();
                quote!(
                    if let #enum_ident::#variant_ident{#fields_idents} = &self {
                        let mut errors = ::serde_valid::validation::MapErrors::new();

                        #fields_validators_tokens

                        if !errors.is_empty() {
                            ::std::result::Result::Err(#errors)?
                        }
                    }
                )
            }
            syn::Fields::Unnamed(fields_unnamed) => {
                let variant_ident = &variant.ident;
                let fields_validators = collect_struct_unnamed_fields_validators(fields_unnamed);
                let mut fields_idents =
                    syn::punctuated::Punctuated::<TokenStream, syn::token::Comma>::new();
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
                let errors = if fields_validators.len() != 1 {
                    fields_errors_tokens()
                } else {
                    new_type_errors_tokens()
                };
                quote!(
                    if let #enum_ident::#variant_ident(#fields_idents) = &self {
                        let mut errors = ::serde_valid::validation::MapErrors::new();

                        #fields_validators_tokens

                        if !errors.is_empty() {
                            ::std::result::Result::Err(#errors)?
                        }
                    }
                )
            }
            syn::Fields::Unit => quote!(),
        };
        enum_validator_tokens.push(variant_tokens);
    }
    enum_validator_tokens.push(quote!(::std::result::Result::Ok(())));
    let validators = TokenStream::from_iter(enum_validator_tokens);
    validators
}
