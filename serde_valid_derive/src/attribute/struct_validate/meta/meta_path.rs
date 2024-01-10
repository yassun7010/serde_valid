use crate::attribute::field_validate::{CustomMessageToken, MetaPathStructValidation, Validator};

#[inline]
pub fn extract_struct_validator_from_meta_path(
    validation_type: MetaPathStructValidation,
    _validation: &syn::Path,
    _custom_message: CustomMessageToken,
) -> Result<Validator, crate::Errors> {
    match validation_type {}
}
