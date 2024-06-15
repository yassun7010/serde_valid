use crate::attribute::{
    common::message_format::MessageFormat, MetaNameValueStructValidation, Validator,
};

#[inline]
pub fn extract_variant_validator_from_meta_name_value(
    validation_type: MetaNameValueStructValidation,
    _validation: &syn::MetaNameValue,
    _message_format: MessageFormat,
) -> Result<Validator, crate::Errors> {
    match validation_type {}
}
