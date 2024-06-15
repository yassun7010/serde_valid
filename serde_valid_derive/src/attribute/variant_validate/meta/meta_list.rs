use crate::attribute::{
    common::message_format::MessageFormat,
    variant_validate::generic::extract_generic_variant_custom_validator, MetaListStructValidation,
    Validator,
};

pub fn extract_variant_validator_from_meta_list(
    validation_type: MetaListStructValidation,
    validation: &syn::MetaList,
    message_format: MessageFormat,
) -> Result<Validator, crate::Errors> {
    match validation_type {
        MetaListStructValidation::Custom => {
            extract_generic_variant_custom_validator(validation, message_format)
        }
    }
}
