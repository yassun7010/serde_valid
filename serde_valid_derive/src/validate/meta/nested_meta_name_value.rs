use crate::serde::rename::RenameMap;
use crate::types::{Field, SingleIdentPath};
use crate::validate::array::{
    extract_array_max_items_validator, extract_array_min_items_validator,
};
use crate::validate::common::{CustomMessage, MetaNameValueValidation};
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
use std::str::FromStr;

pub fn extract_validator_from_nested_meta_name_value(
    field: &impl Field,
    _attribute: &syn::Attribute,
    syn::MetaNameValue {
        path: validation_name,
        lit: validation_value,
        ..
    }: &syn::MetaNameValue,
    custom_message: CustomMessage,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let validation_name_ident = SingleIdentPath::new(validation_name).ident();
    match MetaNameValueValidation::from_str(&validation_name_ident.to_string()) {
        Ok(MetaNameValueValidation::Minimum) => {
            extract_numeric_minimum_validator(field, validation_value, custom_message, rename_map)
        }
        Ok(MetaNameValueValidation::Maximum) => {
            extract_numeric_maximum_validator(field, validation_value, custom_message, rename_map)
        }
        Ok(MetaNameValueValidation::ExclusiveMinimum) => {
            extract_numeric_exclusive_minimum_validator(
                field,
                validation_value,
                custom_message,
                rename_map,
            )
        }
        Ok(MetaNameValueValidation::ExclusiveMaximum) => {
            extract_numeric_exclusive_maximum_validator(
                field,
                validation_value,
                custom_message,
                rename_map,
            )
        }
        Ok(MetaNameValueValidation::MinLength) => {
            extract_string_min_length_validator(field, validation_value, custom_message, rename_map)
        }
        Ok(MetaNameValueValidation::MaxLength) => {
            extract_string_max_length_validator(field, validation_value, custom_message, rename_map)
        }
        Ok(MetaNameValueValidation::MinItems) => {
            extract_array_min_items_validator(field, validation_value, custom_message, rename_map)
        }
        Ok(MetaNameValueValidation::MaxItems) => {
            extract_array_max_items_validator(field, validation_value, custom_message, rename_map)
        }
        Ok(MetaNameValueValidation::MinProperties) => extract_object_min_properties_validator(
            field,
            validation_value,
            custom_message,
            rename_map,
        ),
        Ok(MetaNameValueValidation::MaxProperties) => extract_object_max_properties_validator(
            field,
            validation_value,
            custom_message,
            rename_map,
        ),
        Ok(MetaNameValueValidation::MultipleOf) => extract_numeric_multiple_of_validator(
            field,
            validation_value,
            custom_message,
            rename_map,
        ),
        Ok(MetaNameValueValidation::Pattern) => {
            extract_string_pattern_validator(field, validation_value, custom_message, rename_map)
        }
        Err(unknown) => Err(vec![crate::Error::validate_unknown_type(
            validation_name,
            &unknown,
            &MetaNameValueValidation::iter()
                .map(|x| x.name())
                .collect::<Vec<_>>(),
        )]),
    }
}
