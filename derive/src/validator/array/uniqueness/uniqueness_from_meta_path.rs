use super::inner_extract_array_uniqueness_validator;
use crate::helper::NamedField;
use crate::validator::Validator;
use quote::quote;

pub fn extract_array_uniqueness_validator_from_path(field: &NamedField) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_array_uniqueness_validator_from_path(
            &option_field,
        )))
    } else {
        let message =
            quote!(::serde_valid::validation::error::UniqueItemsErrorParams::to_default_message);
        Validator::Normal(inner_extract_array_uniqueness_validator(
            field.ident(),
            message,
        ))
    }
}
