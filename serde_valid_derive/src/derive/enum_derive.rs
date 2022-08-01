use super::named_struct_derive::collect_named_fields_validators_list;
use super::unnamed_struct_derive::collect_unnamed_fields_validators_list;
use crate::error::{array_errors_tokens, new_type_errors_tokens, object_errors_tokens};
use crate::rule::{collect_rules_from_named_struct, collect_rules_from_unnamed_struct};
use crate::serde::rename::collect_serde_rename_map;
use crate::types::CommaSeparatedTokenStreams;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::iter::FromIterator;

pub type Variants = syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>;

pub fn expand_enum_validate_derive(
    input: &syn::DeriveInput,
    variants: &Variants,
) -> Result<TokenStream, crate::Errors> {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let mut errors = vec![];

    let validations_and_rules =
        TokenStream::from_iter(variants.iter().enumerate().map(|(index, variant)| {
            match &variant.fields {
                syn::Fields::Named(named_fields) => {
                    match expand_enum_variant_named_fields(index, ident, variant, named_fields) {
                        Ok(variant_varidates_and_rules) => variant_varidates_and_rules,
                        Err(variant_errors) => {
                            errors.extend(variant_errors);
                            quote!()
                        }
                    }
                }
                syn::Fields::Unnamed(unnamed_fields) => {
                    match expand_enum_variant_unnamed_fields_varidation(
                        index,
                        ident,
                        variant,
                        unnamed_fields,
                    ) {
                        Ok(variant_varidates_and_rules) => variant_varidates_and_rules,
                        Err(variant_errors) => {
                            errors.extend(variant_errors);
                            quote!()
                        }
                    }
                }
                syn::Fields::Unit => quote!(),
            }
        }));

    if errors.is_empty() {
        Ok(quote!(
            impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
                fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
                    #validations_and_rules

                    Ok(())
                }
            }
        ))
    } else {
        Err(errors)
    }
}

fn expand_enum_variant_named_fields(
    index: usize,
    ident: &syn::Ident,
    variant: &syn::Variant,
    named_fields: &syn::FieldsNamed,
) -> Result<TokenStream, crate::Errors> {
    let mut errors = vec![];

    let variant_ident = &variant.ident;
    let mut fields_idents = CommaSeparatedTokenStreams::new();
    let else_token = make_else_token(index);
    let rename_map = collect_serde_rename_map(named_fields);

    let (rule_fields, rules) = match collect_rules_from_named_struct(&variant.attrs) {
        Ok(field_rules) => field_rules,
        Err(variant_errors) => {
            errors.extend(variant_errors.into_iter());
            (HashSet::new(), quote!())
        }
    };

    let validates = match collect_named_fields_validators_list(named_fields, &rename_map) {
        Ok(field_validators_list) => {
            TokenStream::from_iter(field_validators_list.iter().map(|validators| {
                let field_ident = validators.ident();

                if let Some(token) = validators.get_tokens() {
                    fields_idents.push(quote!(#field_ident));
                    quote!(#token)
                } else {
                    if rule_fields.contains(field_ident) {
                        fields_idents.push(quote!(#field_ident));
                    } else {
                        fields_idents.push(quote!(#field_ident: _));
                    }
                    quote!()
                }
            }))
        }
        Err(fields_errors) => {
            errors.extend(fields_errors.into_iter());
            quote!()
        }
    };

    let variant_errors = object_errors_tokens();

    if errors.is_empty() {
        Ok(quote!(
            #else_token if let #ident::#variant_ident{#fields_idents} = &self {
                let mut __rule_vec_errors = ::serde_valid::validation::VecErrors::new();
                let mut __property_vec_errors_map = ::serde_valid::validation::PropertyVecErrorsMap::new();

                #validates
                #rules

                if !(__rule_vec_errors.is_empty() && __property_vec_errors_map.is_empty()) {
                    Err(#variant_errors)?
                }
            }
        ))
    } else {
        Err(errors)
    }
}

fn expand_enum_variant_unnamed_fields_varidation(
    index: usize,
    ident: &syn::Ident,
    variant: &syn::Variant,
    unnamed_fields: &syn::FieldsUnnamed,
) -> Result<TokenStream, crate::Errors> {
    let mut errors = vec![];

    let variant_ident = &variant.ident;
    let mut fields_idents = CommaSeparatedTokenStreams::new();
    let else_token = make_else_token(index);

    let (rule_fields, rules) = match collect_rules_from_unnamed_struct(&variant.attrs) {
        Ok(field_rules) => field_rules,
        Err(variant_errors) => {
            errors.extend(variant_errors.into_iter());
            (HashSet::new(), quote!())
        }
    };

    let validates = match collect_unnamed_fields_validators_list(unnamed_fields) {
        Ok(field_validators_list) => {
            TokenStream::from_iter(field_validators_list.iter().map(|validators| {
                let field_ident = validators.ident();

                if let Some(token) = validators.get_tokens() {
                    fields_idents.push(quote!(#field_ident));
                    quote!(#token)
                } else {
                    if rule_fields.contains(field_ident) {
                        fields_idents.push(quote!(#field_ident));
                    } else {
                        fields_idents.push(quote!(_));
                    }
                    quote!()
                }
            }))
        }
        Err(fields_errors) => {
            errors.extend(fields_errors.into_iter());
            quote!()
        }
    };

    let variant_errors = if unnamed_fields.unnamed.len() != 1 {
        array_errors_tokens()
    } else {
        new_type_errors_tokens()
    };

    if errors.is_empty() {
        Ok(quote!(
            #else_token if let #ident::#variant_ident(#fields_idents) = &self {
                let mut __rule_vec_errors = ::serde_valid::validation::VecErrors::new();
                let mut __item_vec_errors_map = ::serde_valid::validation::ItemVecErrorsMap::new();

                #validates
                #rules

                if !(__rule_vec_errors.is_empty() && __item_vec_errors_map.is_empty()) {
                    Err(#variant_errors)?
                }
            }
        ))
    } else {
        Err(errors)
    }
}

fn make_else_token(index: usize) -> TokenStream {
    if index == 0 {
        quote!()
    } else {
        quote!(else)
    }
}
