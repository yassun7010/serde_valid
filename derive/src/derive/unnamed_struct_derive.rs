use crate::error::{fields_errors_tokens, new_type_errors_tokens};
use crate::rule::collect_rules_from_unnamed_struct;
use crate::types::{Field, UnnamedField};
use crate::validator::{extract_meta_validator, FieldValidators};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::iter::FromIterator;
use syn::parse_quote;

pub fn expand_unnamed_struct_derive(
    input: &syn::DeriveInput,
    fields: &syn::FieldsUnnamed,
) -> Result<TokenStream, crate::Errors> {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let mut errors: crate::Errors = vec![];

    let rules = TokenStream::from_iter(
        collect_rules_from_unnamed_struct(&input.attrs).unwrap_or_else(|rule_errors| {
            errors.extend(rule_errors.into_iter());
            vec![quote!()]
        }),
    );

    let validators = match collect_unnamed_fields_validators(fields) {
        Ok(field_validators) => TokenStream::from_iter(
            field_validators
                .iter()
                .map(|validator| validator.generate_tokens()),
        ),
        Err(validation_errors) => {
            errors.extend(validation_errors.into_iter());
            quote!()
        }
    };

    let fields_errors = if fields.unnamed.len() != 1 {
        fields_errors_tokens()
    } else {
        new_type_errors_tokens()
    };

    if errors.is_empty() {
        Ok(quote!(
            impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
                fn validate(&self) -> Result<(), ::serde_valid::validation::Errors> {
                    let mut __errors = ::serde_valid::validation::MapErrors::new();

                    #validators
                    #rules

                    if __errors.is_empty() {
                        Result::Ok(())
                    } else {
                        Result::Err(#fields_errors)
                    }
                }
            }
        ))
    } else {
        Err(errors)
    }
}

pub fn collect_unnamed_fields_validators<'a>(
    fields: &'a syn::FieldsUnnamed,
) -> Result<Vec<FieldValidators<'a, UnnamedField<'a>>>, crate::Errors> {
    let mut errors = vec![];

    let validators = fields
        .unnamed
        .iter()
        .enumerate()
        .filter_map(|field| match collect_unnamed_field_validators(field) {
            Ok(validators) => Some(validators),
            Err(ref mut error) => {
                errors.append(error);
                None
            }
        })
        .collect();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(validators)
}

fn collect_unnamed_field_validators<'a>(
    (index, field): (usize, &'a syn::Field),
) -> Result<FieldValidators<'a, UnnamedField<'a>>, crate::Errors> {
    let mut errors: crate::Errors = vec![];

    let unnamed_field = UnnamedField::new(index, field);
    let validators = unnamed_field
        .attrs()
        .iter()
        .filter(|attribute| attribute.path == parse_quote!(validate))
        .filter_map(
            |attribute| match extract_meta_validator(&unnamed_field, attribute) {
                Ok(validator) => Some(validator),
                Err(error) => {
                    errors.push(error);
                    None
                }
            },
        )
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        return Err(errors);
    }

    let mut field_validators = FieldValidators::new(Cow::Owned(unnamed_field));
    validators
        .into_iter()
        .for_each(|validator| field_validators.push(validator));

    Ok(field_validators)
}
