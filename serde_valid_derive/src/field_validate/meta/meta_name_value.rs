use crate::field_validate::array::{
    extract_array_max_items_validator, extract_array_min_items_validator,
};
use crate::field_validate::common::{get_lit, CustomMessageToken, MetaNameValueFieldValidation};
use crate::field_validate::numeric::{
    extract_numeric_exclusive_maximum_validator, extract_numeric_exclusive_minimum_validator,
    extract_numeric_maximum_validator, extract_numeric_minimum_validator,
    extract_numeric_multiple_of_validator,
};
use crate::field_validate::object::{
    extract_object_max_properties_validator, extract_object_min_properties_validator,
};
use crate::field_validate::string::{
    extract_string_max_length_validator, extract_string_min_length_validator,
    extract_string_pattern_validator,
};
use crate::field_validate::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;

pub fn extract_field_validator_from_meta_name_value(
    field: &impl Field,
    validation_type: MetaNameValueFieldValidation,
    validation: &syn::MetaNameValue,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let validation_value = get_lit(&validation.value)?;

    match validation_type {
        MetaNameValueFieldValidation::Minimum => {
            extract_numeric_minimum_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueFieldValidation::Maximum => {
            extract_numeric_maximum_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueFieldValidation::ExclusiveMinimum => {
            extract_numeric_exclusive_minimum_validator(
                field,
                validation_value,
                custom_message,
                rename_map,
            )
        }
        MetaNameValueFieldValidation::ExclusiveMaximum => {
            extract_numeric_exclusive_maximum_validator(
                field,
                validation_value,
                custom_message,
                rename_map,
            )
        }
        MetaNameValueFieldValidation::MinLength => {
            extract_string_min_length_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueFieldValidation::MaxLength => {
            extract_string_max_length_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueFieldValidation::MinItems => {
            extract_array_min_items_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueFieldValidation::MaxItems => {
            extract_array_max_items_validator(field, validation_value, custom_message, rename_map)
        }
        MetaNameValueFieldValidation::MinProperties => extract_object_min_properties_validator(
            field,
            validation_value,
            custom_message,
            rename_map,
        ),
        MetaNameValueFieldValidation::MaxProperties => extract_object_max_properties_validator(
            field,
            validation_value,
            custom_message,
            rename_map,
        ),
        MetaNameValueFieldValidation::MultipleOf => extract_numeric_multiple_of_validator(
            field,
            validation_value,
            custom_message,
            rename_map,
        ),
        MetaNameValueFieldValidation::Pattern => {
            extract_string_pattern_validator(field, validation_value, custom_message, rename_map)
        }
    }
}
