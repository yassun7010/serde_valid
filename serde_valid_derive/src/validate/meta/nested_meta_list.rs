use crate::serde::rename::RenameMap;
use crate::types::Field;
use crate::validate::common::{CustomMessageToken, MetaListValidation};
use crate::validate::generic::{
    extract_generic_custom_validator, extract_generic_enumerate_validator,
};
use crate::validate::Validator;

pub fn extract_validator_from_nested_meta_list(
    field: &impl Field,
    validation_type: MetaListValidation,
    validation: &syn::MetaList,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    match validation_type {
        MetaListValidation::Enumerate => {
            extract_generic_enumerate_validator(field, validation, custom_message, rename_map)
        }
        MetaListValidation::Custom => {
            extract_generic_custom_validator(field, validation, rename_map)
        }
    }
}
