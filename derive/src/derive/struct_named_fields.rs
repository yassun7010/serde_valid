use crate::abort::abort_invalid_attribute_on_field;
use crate::types::NamedField;
use crate::validator::{extract_meta_validator, FieldValidators};
use proc_macro2::TokenStream;
use ref_cast::RefCast;
use std::iter::FromIterator;
use syn::parse_quote;
use syn::spanned::Spanned;

pub fn expand_struct_named_fields_validators_tokens(fields: &syn::FieldsNamed) -> TokenStream {
    TokenStream::from_iter(
        collect_struct_named_fields_validators(fields)
            .iter()
            .map(|validator| validator.generate_tokens()),
    )
}

pub fn collect_struct_named_fields_validators(fields: &syn::FieldsNamed) -> Vec<FieldValidators> {
    let mut struct_validators = vec![];
    for field in fields.named.iter() {
        let mut field_validators = FieldValidators::new(field.clone());
        let named_field = NamedField::ref_cast(field);
        let field_ident = named_field.ident();
        for attribute in named_field.attrs() {
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
