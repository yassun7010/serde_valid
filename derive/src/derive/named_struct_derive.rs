use crate::errors::{fields_errors_tokens, Errors};
use crate::types::{Field, NamedField};
use crate::validator::{extract_meta_validator, FieldValidators};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::iter::FromIterator;
use syn::parse_quote;

pub fn expand_named_struct_derive(
    input: &syn::DeriveInput,
    fields: &syn::FieldsNamed,
) -> Result<TokenStream, Errors> {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let validators = TokenStream::from_iter(
        collect_named_fields_validators(fields)?
            .iter()
            .map(|validator| validator.generate_tokens()),
    );
    let errors = fields_errors_tokens();

    Ok(quote!(
        impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
            fn validate(&self) -> Result<(), ::serde_valid::validation::Errors> {
                let mut __errors = ::serde_valid::validation::MapErrors::new();

                #validators

                if __errors.is_empty() {
                    Result::Ok(())
                } else {
                    Result::Err(#errors)
                }
            }
        }
    ))
}

pub fn collect_named_fields_validators<'a>(
    fields: &'a syn::FieldsNamed,
) -> Result<Vec<FieldValidators<'a, NamedField<'a>>>, Errors> {
    let mut struct_validators = vec![];
    for field in fields.named.iter() {
        let named_field = NamedField::new(field);
        let validators = named_field
            .attrs()
            .iter()
            .filter(|attribute| attribute.path == parse_quote!(validate))
            .map(|attribute| extract_meta_validator(&named_field, attribute))
            .collect::<Result<Vec<_>, _>>()?;

        let mut field_validators = FieldValidators::new(Cow::Owned(named_field));
        validators
            .into_iter()
            .for_each(|validator| field_validators.push(validator));

        struct_validators.push(field_validators)
    }

    Ok(struct_validators)
}
