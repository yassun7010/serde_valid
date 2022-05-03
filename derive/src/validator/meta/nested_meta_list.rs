use crate::types::{Field, SingleIdentPath};
use crate::validator::array::{
    extract_array_items_validator, extract_array_unique_items_validator_from_meta_list,
};
use crate::validator::generic::{
    extract_generic_custom_validator, extract_generic_enumerate_validator,
};
use crate::validator::numeric::{
    extract_numeric_multiple_of_validator_from_meta_list, extract_numeric_range_validator,
};
use crate::validator::object::extract_object_properties_validator;
use crate::validator::string::{
    extract_string_length_validator, extract_string_pattern_of_validator_from_meta_list,
};
use crate::validator::Validator;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
) -> Result<Validator, Vec<syn::Error>> {
    let syn::MetaList {
        path: validation_name,
        ..
    } = validation_list;
    let validation_ident = SingleIdentPath::new(&validation_name).ident();

    match validation_ident.to_string().as_ref() {
        "range" => {
            return Ok(extract_numeric_range_validator(
                field,
                attribute,
                validation_list,
            ))
        }
        "multiple_of" => {
            return Ok(extract_numeric_multiple_of_validator_from_meta_list(
                field,
                attribute,
                validation_list,
            ))
        }
        "length" => {
            return Ok(extract_string_length_validator(
                field,
                attribute,
                validation_list,
            ))
        }
        "pattern" => {
            return Ok(extract_string_pattern_of_validator_from_meta_list(
                field,
                attribute,
                validation_list,
            ))
        }
        "items" => {
            return Ok(extract_array_items_validator(
                field,
                attribute,
                validation_list,
            ))
        }
        "unique_items" => {
            return Ok(extract_array_unique_items_validator_from_meta_list(
                field,
                attribute,
                validation_list,
            ))
        }
        "properties" => {
            return Ok(extract_object_properties_validator(
                field,
                attribute,
                validation_list,
            ))
        }
        "enumerate" => {
            return Ok(extract_generic_enumerate_validator(
                field,
                attribute,
                validation_list,
            ))
        }
        "custom" => {
            return Ok(extract_generic_custom_validator(
                field,
                attribute,
                validation_list,
            ))
        }
        v => Err(vec![syn::Error::new(
            validation_name.span(),
            format!("Unexpected list validator: {v:?}"),
        )]),
    }
}
