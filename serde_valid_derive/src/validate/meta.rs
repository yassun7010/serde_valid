mod meta_list;
mod meta_path;
mod nested_meta_list;
mod nested_meta_name_value;
mod nested_meta_path;

use crate::serde::rename::RenameMap;
use crate::types::Field;
use crate::validate::Validator;
use meta_path::extract_field_validator_from_meta_path;

use self::meta_list::extract_field_validator_from_meta_list;

pub fn extract_field_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    match &attribute.meta {
        syn::Meta::List(list) => {
            extract_field_validator_from_meta_list(field, attribute, list, rename_map)
        }
        syn::Meta::Path(_) => extract_field_validator_from_meta_path(field, rename_map),
        syn::Meta::NameValue(name_value) => {
            Err(vec![crate::Error::validate_meta_name_value_not_support(
                name_value,
            )])
        }
    }
}
