use super::named_struct_derive::collect_named_fields_validators_list;
use super::unnamed_struct_derive::collect_unnamed_fields_validators_list;
use crate::attribute::rule::{collect_rules_from_named_struct, collect_rules_from_unnamed_struct};
use crate::attribute::variant_validate::collect_variant_custom_from_named_variant;
use crate::error::{array_errors_tokens, new_type_errors_tokens, object_errors_tokens};
use crate::output_stream::OutputStream;
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
) -> Result<OutputStream, crate::Errors> {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let mut errors = vec![];

    let validations = variants
        .into_iter()
        .map(|variant| match &variant.fields {
            syn::Fields::Named(named_fields) => {
                match expand_enum_variant_named_fields_validation(ident, variant, named_fields) {
                    Ok(variant_varidates_and_rules) => variant_varidates_and_rules,
                    Err(variant_errors) => {
                        errors.extend(variant_errors);
                        OutputStream::new()
                    }
                }
            }
            syn::Fields::Unnamed(unnamed_fields) => {
                match expand_enum_variant_unnamed_fields_varidation(ident, variant, unnamed_fields)
                {
                    Ok(variant_varidates_and_rules) => variant_varidates_and_rules,
                    Err(variant_errors) => {
                        errors.extend(variant_errors);
                        OutputStream::new()
                    }
                }
            }
            syn::Fields::Unit => OutputStream::new(),
        })
        .collect::<Vec<_>>();

    let validations_and_rules = TokenStream::from_iter(
        validations
            .iter()
            .map(|variant| variant.output.clone())
            .collect::<Vec<_>>(),
    );
    let warnings = validations
        .into_iter()
        .flat_map(|variant| variant.warnings)
        .collect::<Vec<_>>();

    if errors.is_empty() {
        Ok(OutputStream {
            output: quote!(
                impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
                    fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
                        #validations_and_rules

                        Ok(())
                    }
                }
            ),
            warnings,
        })
    } else {
        Err(errors)
    }
}

fn expand_enum_variant_named_fields_validation(
    ident: &syn::Ident,
    variant: &syn::Variant,
    named_fields: &syn::FieldsNamed,
) -> Result<OutputStream, crate::Errors> {
    let mut errors = vec![];

    let variant_ident = &variant.ident;
    let mut fields_idents = CommaSeparatedTokenStreams::new();
    let rename_map = collect_serde_rename_map(named_fields);

    let (
        rule_fields,
        OutputStream {
            output: rules,
            warnings,
        },
    ) = match collect_rules_from_named_struct(&variant.attrs) {
        Ok(field_rules) => field_rules,
        Err(variant_errors) => {
            errors.extend(variant_errors);
            (HashSet::new(), OutputStream::new())
        }
    };

    let enum_validations = match collect_variant_custom_from_named_variant(&variant.attrs) {
        Ok(validations) => TokenStream::from_iter(validations),
        Err(rule_errors) => {
            errors.extend(rule_errors);
            quote!()
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
            errors.extend(fields_errors);
            quote!()
        }
    };

    let variant_errors = object_errors_tokens();

    if errors.is_empty() {
        Ok(OutputStream {
            output: quote!(
                if let #ident::#variant_ident{#fields_idents} = &self {
                    let mut __rule_vec_errors = ::serde_valid::validation::VecErrors::new();
                    let mut __property_vec_errors_map = ::serde_valid::validation::PropertyVecErrorsMap::new();

                    #validates
                    #enum_validations
                    #rules

                    if !(__rule_vec_errors.is_empty() && __property_vec_errors_map.is_empty()) {
                        Err(#variant_errors)?
                    }
                }
            ),
            warnings,
        })
    } else {
        Err(errors)
    }
}

fn expand_enum_variant_unnamed_fields_varidation(
    ident: &syn::Ident,
    variant: &syn::Variant,
    unnamed_fields: &syn::FieldsUnnamed,
) -> Result<OutputStream, crate::Errors> {
    let mut errors = vec![];

    let variant_ident = &variant.ident;
    let mut fields_idents = CommaSeparatedTokenStreams::new();

    let (rule_fields, rules) = match collect_rules_from_unnamed_struct(&variant.attrs) {
        Ok(field_rules) => field_rules,
        Err(variant_errors) => {
            errors.extend(variant_errors);
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
            errors.extend(fields_errors);
            quote!()
        }
    };

    let variant_errors = if unnamed_fields.unnamed.len() != 1 {
        array_errors_tokens()
    } else {
        new_type_errors_tokens()
    };

    if errors.is_empty() {
        Ok(OutputStream {
            output: quote!(
                if let #ident::#variant_ident(#fields_idents) = &self {
                    let mut __rule_vec_errors = ::serde_valid::validation::VecErrors::new();
                    let mut __item_vec_errors_map = ::serde_valid::validation::ItemVecErrorsMap::new();

                    #validates
                    #rules

                    if !(__rule_vec_errors.is_empty() && __item_vec_errors_map.is_empty()) {
                        Err(#variant_errors)?
                    }
                }
            ),
            warnings: vec![],
        })
    } else {
        Err(errors)
    }
}
