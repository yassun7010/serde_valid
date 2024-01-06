use crate::serde::rename::RenameMap;
use crate::types::Field;
use crate::validate::array::{
    extract_array_max_items_validator, extract_array_min_items_validator,
};
use crate::validate::common::{get_lit, CustomMessageToken, MetaNameValueValidation};
use crate::validate::numeric::{
    extract_numeric_exclusive_maximum_validator, extract_numeric_exclusive_minimum_validator,
    extract_numeric_maximum_validator, extract_numeric_minimum_validator,
    extract_numeric_multiple_of_validator,
};
use crate::validate::object::{
    extract_object_max_properties_validator, extract_object_min_properties_validator,
};
use crate::validate::string::{
    extract_string_max_length_validator, extract_string_min_length_validator,
    extract_string_pattern_validator,
};
use crate::validate::Validator;

pub fn extract_validator_from_nested_meta_name_value(
    field: &impl Field,
    validation_type: MetaNameValueValidation,
    validation: &syn::MetaNameValue,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let validation_value = get_lit(&validation.value)?;

    match validation_type {
        MetaNameValueValidation::Minimum => {
            extract_numeric_minimum_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueValidation::Maximum => {
            extract_numeric_maximum_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueValidation::ExclusiveMinimum => extract_numeric_exclusive_minimum_validator(
            field,
            validation_value,
            custom_message,
            rename_map,
        ),
        MetaNameValueValidation::ExclusiveMaximum => extract_numeric_exclusive_maximum_validator(
            field,
            validation_value,
            custom_message,
            rename_map,
        ),
        MetaNameValueValidation::MinLength => {
            extract_string_min_length_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueValidation::MaxLength => {
            extract_string_max_length_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueValidation::MinItems => {
            extract_array_min_items_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueValidation::MaxItems => {
            extract_array_max_items_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueValidation::MinProperties => extract_object_min_properties_validator(
            field,
            validation_value,
            custom_message,
            rename_map,
        ),
        MetaNameValueValidation::MaxProperties => extract_object_max_properties_validator(
            field,
            validation_value,
            custom_message,
            rename_map,
        ),
        MetaNameValueValidation::MultipleOf => extract_numeric_multiple_of_validator(
            field,
            validation_value,
            custom_message,
            rename_map,
        ),
        MetaNameValueValidation::Pattern => {
            extract_string_pattern_validator(field, validation_value, custom_message, rename_map)
        }
    }
}
