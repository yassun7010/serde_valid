use crate::attribute::field_validate::{CustomMessageToken, MetaListStructValidation, Validator};
use crate::attribute::struct_validate::generic::extract_generic_struct_custom_validator;

pub fn extract_struct_validator_from_meta_list(
    validation_type: MetaListStructValidation,
    validation: &syn::MetaList,
    _custom_message: CustomMessageToken,
) -> Result<Validator, crate::Errors> {
    match validation_type {
        MetaListStructValidation::Custom => extract_generic_struct_custom_validator(validation),
    }
}
