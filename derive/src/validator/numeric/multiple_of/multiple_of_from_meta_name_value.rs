use super::{inner_extract_numeric_multiple_of_validator, VALIDATION_LABEL};
use crate::types::Field;
use crate::validator::common::get_numeric;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_numeric_multiple_of_validator_from_meta_name_value<F: Field>(
    field: &F,
    lit: &syn::Lit,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(
            extract_numeric_multiple_of_validator_from_meta_name_value(&array_field, lit),
        ))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(
            extract_numeric_multiple_of_validator_from_meta_name_value(&option_field, lit),
        ))
    } else {
        Validator::Normal(
            inner_extract_numeric_multiple_of_validator_from_meta_name_value(
                field.name(),
                field.ident(),
                lit,
            ),
        )
    }
}

fn inner_extract_numeric_multiple_of_validator_from_meta_name_value(
    field_name: &str,
    field_ident: &syn::Ident,
    lit: &syn::Lit,
) -> TokenStream {
    let multiple_of = get_numeric(VALIDATION_LABEL, field_ident, lit);
    let message = quote!(::serde_valid::validation::error::MultipleOfParams::to_default_message);
    inner_extract_numeric_multiple_of_validator(field_name, field_ident, multiple_of, message)
}
