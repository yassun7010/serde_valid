use crate::abort::abort_invalid_attribute_on_field;
use crate::errors::fields_errors_tokens;
use crate::types::{Field, NamedField};
use crate::validator::{extract_meta_validator, FieldValidators};
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;
use syn::parse_quote;
use syn::spanned::Spanned;

pub fn expand_struct_named_fields_validate(
    input: &syn::DeriveInput,
    fields: &syn::FieldsNamed,
) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let validators = TokenStream::from_iter(
        collect_struct_named_fields_validators(fields)
            .iter()
            .map(|validator| validator.generate_tokens()),
    );
    let errors = fields_errors_tokens();

    quote!(
        impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
            fn validate(
                &self
            ) -> ::std::result::Result<(), ::serde_valid::validation::Errors> {
                let mut errors = ::serde_valid::validation::MapErrors::new();

                #validators

                if errors.is_empty() {
                    ::std::result::Result::Ok(())
                } else {
                    ::std::result::Result::Err(#errors)
                }
            }
        }
    )
}

pub fn collect_struct_named_fields_validators(
    fields: &syn::FieldsNamed,
) -> Vec<FieldValidators<NamedField>> {
    let mut struct_validators = vec![];
    for field in fields.named.iter() {
        let mut field_validators = FieldValidators::new(NamedField::new(field.to_owned()));
        let named_field = &NamedField::new(field.to_owned());
        for attribute in named_field.attrs() {
            if attribute.path != parse_quote!(validate) {
                continue;
            }
            let validator = extract_meta_validator(named_field, attribute);
            match validator {
                Some(validator) => field_validators.push(validator),
                None => abort_invalid_attribute_on_field(
                    named_field,
                    attribute.span(),
                    "it needs at least one validator",
                ),
            }
        }
        struct_validators.push(field_validators)
    }

    struct_validators
}
