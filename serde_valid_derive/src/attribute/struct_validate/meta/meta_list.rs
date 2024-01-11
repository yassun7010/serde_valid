use crate::attribute::{
    common::message_format::MessageFormat,
    struct_validate::generic::extract_generic_struct_custom_validator, MetaListStructValidation,
    Validator,
};

pub fn extract_struct_validator_from_meta_list(
    validation_type: MetaListStructValidation,
    validation: &syn::MetaList,
    message_format: MessageFormat,
) -> Result<Validator, crate::Errors> {
    match validation_type {
        MetaListStructValidation::Custom => {
            extract_generic_struct_custom_validator(validation, message_format)
        }
    }
}
