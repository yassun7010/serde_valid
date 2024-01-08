use crate::attribute::field_validate::{MetaNameValueStructValidation, Validator};

#[inline]
pub fn extract_struct_validator_from_meta_name_value(
    validation_type: MetaNameValueStructValidation,
    _validation: &syn::MetaNameValue,
) -> Result<Validator, crate::Errors> {
    match validation_type {}
}
