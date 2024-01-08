use crate::field_validate::array::extract_array_unique_items_validator;
use crate::field_validate::common::{CustomMessageToken, MetaPathFieldValidation};
use crate::field_validate::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;

pub fn extract_field_validator_from_nested_meta_path(
    field: &impl Field,
    validation_type: MetaPathFieldValidation,
    _validation: &syn::Path,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    match validation_type {
        MetaPathFieldValidation::UniqueItems => Ok(extract_array_unique_items_validator(
            field,
            custom_message,
            rename_map,
        )),
    }
}
