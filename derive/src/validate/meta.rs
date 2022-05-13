mod meta_list;
mod meta_path;
mod nested_meta_list;
mod nested_meta_name_value;
mod nested_meta_path;

use std::collections::HashMap;

use crate::types::Field;
use crate::validate::Validator;
use meta_path::extract_validator_from_meta_path;

use self::meta_list::extract_validator_from_meta_list;

pub fn extract_meta_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
    rename_map: &HashMap<String, String>,
) -> Result<Validator, crate::Errors> {
    match attribute.parse_meta() {
        Ok(syn::Meta::List(list)) => {
            extract_validator_from_meta_list(field, attribute, &list, rename_map)
        }
        Ok(syn::Meta::Path(_)) => extract_validator_from_meta_path(field, rename_map),
        Ok(syn::Meta::NameValue(name_value)) => {
            Err(vec![crate::Error::validate_meta_name_value_not_support(
                name_value,
            )])
        }
        Err(error) => Err(vec![crate::Error::validate_attribute_parse_error(
            attribute, &error,
        )]),
    }
}
