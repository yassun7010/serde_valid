use crate::errors::Error;
use crate::types::{Field, SingleIdentPath};
use crate::validator::array::{
    extract_array_max_items_validator, extract_array_min_items_validator,
};
use crate::validator::numeric::{
    extract_numeric_exclusive_maximum_validator, extract_numeric_exclusive_minimum_validator,
    extract_numeric_maximum_validator, extract_numeric_minimum_validator,
    extract_numeric_multiple_of_validator_from_meta_name_value,
};
use crate::validator::object::{
    extract_object_max_properties_validator, extract_object_min_properties_validator,
};
use crate::validator::string::{
    extract_string_max_length_validator, extract_string_min_length_validator,
    extract_string_pattern_validator_from_meta_name_value,
};
use crate::validator::Validator;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_name_value(
    field: &impl Field,
    _attribute: &syn::Attribute,
    syn::MetaNameValue {
        path: validation_name,
        lit: validation_value,
        ..
    }: &syn::MetaNameValue,
) -> Result<Validator, Error> {
    let validation_name_ident = SingleIdentPath::new(validation_name).ident();
    match validation_name_ident.to_string().as_ref() {
        "minimum" => return Ok(extract_numeric_minimum_validator(field, validation_value)),
        "maximum" => return Ok(extract_numeric_maximum_validator(field, validation_value)),
        "exclusive_minimum" => {
            return Ok(extract_numeric_exclusive_minimum_validator(
                field,
                validation_value,
            ))
        }
        "exclusive_maximum" => {
            return Ok(extract_numeric_exclusive_maximum_validator(
                field,
                validation_value,
            ))
        }
        "min_length" => return Ok(extract_string_min_length_validator(field, validation_value)),
        "max_length" => return Ok(extract_string_max_length_validator(field, validation_value)),
        "min_items" => return Ok(extract_array_min_items_validator(field, validation_value)),
        "max_items" => return Ok(extract_array_max_items_validator(field, validation_value)),
        "min_properties" => {
            return Ok(extract_object_min_properties_validator(
                field,
                validation_value,
            ))
        }
        "max_properties" => {
            return Ok(extract_object_max_properties_validator(
                field,
                validation_value,
            ))
        }
        "multiple_of" => {
            return Ok(extract_numeric_multiple_of_validator_from_meta_name_value(
                field,
                validation_value,
            ))
        }
        "pattern" => {
            return Ok(extract_string_pattern_validator_from_meta_name_value(
                field,
                validation_value,
            ))
        }
        v => Err(Error::new(
            validation_name.span(),
            format!("Unexpected name value validator: {v:?}"),
        )),
    }
}
