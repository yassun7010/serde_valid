use super::{inner_extract_string_pattern_validator, VALIDATION_LABEL};
use crate::types::Field;
use crate::validator::common::get_str;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_string_pattern_validator_from_meta_name_value<F: Field>(
    field: &F,
    validation_value: &syn::Lit,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(
            extract_string_pattern_validator_from_meta_name_value(&array_field, validation_value),
        ))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(
            extract_string_pattern_validator_from_meta_name_value(&option_field, validation_value),
        ))
    } else {
        Validator::Normal(inner_extract_string_pattern_validator_from_meta_name_value(
            field,
            validation_value,
        ))
    }
}

fn inner_extract_string_pattern_validator_from_meta_name_value<F: Field>(
    field: &F,
    validation_value: &syn::Lit,
) -> TokenStream {
    let pattern = get_str(VALIDATION_LABEL, field, validation_value);
    let message = quote!(::serde_valid::validation::error::PatternParams::to_default_message);
    inner_extract_string_pattern_validator(field, &pattern, &message)
}
