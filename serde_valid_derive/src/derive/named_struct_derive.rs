use crate::error::object_errors_tokens;
use crate::rule::collect_rules_from_named_struct;
use crate::serde::rename::{collect_serde_rename_map, RenameMap};
use crate::types::{Field, NamedField};
use crate::validate::{extract_meta_validator, FieldValidators, Validator};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::collections::HashSet;
use std::iter::FromIterator;
use syn::parse_quote;

pub fn expand_named_struct_derive(
    input: &syn::DeriveInput,
    fields: &syn::FieldsNamed,
) -> Result<TokenStream, crate::Errors> {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let rename_map = collect_serde_rename_map(fields);

    let mut errors = vec![];

    let (rule_fields, rules) = match collect_rules_from_named_struct(&input.attrs) {
        Ok((rule_fields, rules)) => (rule_fields, TokenStream::from_iter(rules)),
        Err(rule_errors) => {
            errors.extend(rule_errors);
            (HashSet::new(), quote!())
        }
    };

    let validates = match collect_named_fields_validators_list(fields, &rename_map) {
        Ok(field_validators) => TokenStream::from_iter(field_validators.iter().map(|validator| {
            if validator.is_empty() && rule_fields.contains(validator.ident()) {
                validator.get_field_variable_token()
            } else {
                validator.generate_tokens()
            }
        })),
        Err(validation_errors) => {
            errors.extend(validation_errors.into_iter());
            quote!()
        }
    };

    let fields_errors = object_errors_tokens();

    if errors.is_empty() {
        Ok(quote!(
            impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
                fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
                    let mut __rule_vec_errors = ::serde_valid::validation::VecErrors::new();
                    let mut __property_vec_errors_map = ::serde_valid::validation::PropertyVecErrorsMap::new();

                    #validates
                    #rules

                    if __rule_vec_errors.is_empty() && __property_vec_errors_map.is_empty() {
                        Ok(())
                    } else {
                        Err(#fields_errors)
                    }
                }
            }
        ))
    } else {
        Err(errors)
    }
}

pub fn collect_named_fields_validators_list<'a>(
    fields: &'a syn::FieldsNamed,
    rename_map: &RenameMap,
) -> Result<Vec<FieldValidators<'a, NamedField<'a>>>, crate::Errors> {
    let mut errors = vec![];

    let validators = fields
        .named
        .iter()
        .filter_map(
            |field| match collect_named_field_validators(field, rename_map) {
                Ok(validators) => Some(validators),
                Err(ref mut error) => {
                    errors.append(error);
                    None
                }
            },
        )
        .collect();

    if errors.is_empty() {
        Ok(validators)
    } else {
        Err(errors)
    }
}

fn collect_named_field_validators<'a>(
    field: &'a syn::Field,
    rename_map: &RenameMap,
) -> Result<FieldValidators<'a, NamedField<'a>>, crate::Errors> {
    let mut errors = vec![];

    let named_field = NamedField::new(field);

    let validators = named_field
        .attrs()
        .iter()
        .filter_map(|attribute| {
            if attribute.path != parse_quote!(validate) {
                return None;
            }
            match extract_meta_validator(&named_field, attribute, rename_map) {
                Ok(validator) => Some(validator),
                Err(validator_error) => {
                    errors.extend(validator_error);
                    None
                }
            }
        })
        .collect::<Vec<Validator>>();

    if !errors.is_empty() {
        return Err(errors);
    }

    let mut field_validators = FieldValidators::new(Cow::Owned(named_field));
    validators
        .into_iter()
        .for_each(|validator| field_validators.push(validator));

    Ok(field_validators)
}
