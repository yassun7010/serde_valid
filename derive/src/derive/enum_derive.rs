use super::named_struct_derive::collect_named_fields_validators;
use super::unnamed_struct_derive::collect_unnamed_fields_validators;
use crate::error::{fields_errors_tokens, new_type_errors_tokens};
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

type Variants = syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>;

pub fn expand_enum_validate_derive(
    input: &syn::DeriveInput,
    variants: &Variants,
) -> Result<TokenStream, crate::Errors> {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let validations = TokenStream::from_iter(
        variants
            .iter()
            .map(|variant| match &variant.fields {
                syn::Fields::Named(named_fields) => {
                    expand_enum_variant_named_fields_validation(ident, variant, named_fields)
                }
                syn::Fields::Unnamed(unnamed_fields) => {
                    expand_enum_variant_unnamed_fields_varidation(ident, variant, unnamed_fields)
                }
                syn::Fields::Unit => Ok(quote!()),
            })
            .collect::<Result<TokenStream, _>>(),
    );

    Ok(quote!(
        impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
            fn validate(&self) -> Result<(), ::serde_valid::validation::Errors> {
                #validations

                Result::Ok(())
            }
        }
    ))
}

fn expand_enum_variant_named_fields_validation(
    ident: &syn::Ident,
    variant: &syn::Variant,
    named_fields: &syn::FieldsNamed,
) -> Result<TokenStream, crate::Errors> {
    let variant_ident = &variant.ident;
    let fields_validators = collect_named_fields_validators(named_fields)?;
    let mut fields_idents = syn::punctuated::Punctuated::<TokenStream, syn::token::Comma>::new();
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

    Ok(quote!(
        if let #ident::#variant_ident{#fields_idents} = &self {
            let mut __errors = ::serde_valid::validation::MapErrors::new();

            #fields_validators_tokens

            if !__errors.is_empty() {
                Result::Err(#errors)?
            }
        }
    ))
}

fn expand_enum_variant_unnamed_fields_varidation(
    ident: &syn::Ident,
    variant: &syn::Variant,
    unnamed_fields: &syn::FieldsUnnamed,
) -> Result<TokenStream, crate::Errors> {
    let variant_ident = &variant.ident;
    let fields_validators = collect_unnamed_fields_validators(unnamed_fields)?;
    let mut fields_idents = syn::punctuated::Punctuated::<TokenStream, syn::token::Comma>::new();
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
    Ok(quote!(
        if let #ident::#variant_ident(#fields_idents) = &self {
            let mut __errors = ::serde_valid::validation::MapErrors::new();

            #fields_validators_tokens

            if !__errors.is_empty() {
                Result::Err(#errors)?
            }
        }
    ))
}
