use crate::helper::{NamedField, SingleIdentPath};
use crate::validator::array::{
    extract_array_items_validator, extract_array_unique_items_validator_from_meta_list,
};
use crate::validator::generic::extract_generic_enumerate_validator;
use crate::validator::numeric::{
    extract_numeric_multiple_of_validator_from_meta_list, extract_numeric_range_validator,
};
use crate::validator::object::extract_object_properties_validator;
use crate::validator::string::{
    extract_string_length_validator, extract_string_pattern_of_validator_from_meta_list,
};
use crate::validator::Validator;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_list(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Option<Validator> {
    let syn::MetaList { path, .. } = meta_list;
    let ident = SingleIdentPath::new(&path).ident();

    match ident.to_string().as_ref() {
        "range" => return Some(extract_numeric_range_validator(field, attribute, meta_list)),
        "multiple_of" => {
            return Some(extract_numeric_multiple_of_validator_from_meta_list(
                field, attribute, meta_list,
            ))
        }
        "length" => return Some(extract_string_length_validator(field, attribute, meta_list)),
        "pattern" => {
            return Some(extract_string_pattern_of_validator_from_meta_list(
                field, attribute, meta_list,
            ))
        }
        "items" => return Some(extract_array_items_validator(field, attribute, meta_list)),
        "unique_items" => {
            return Some(extract_array_unique_items_validator_from_meta_list(
                field, attribute, meta_list,
            ))
        }
        "properties" => {
            return Some(extract_object_properties_validator(
                field, attribute, meta_list,
            ))
        }
        "enumerate" => {
            return Some(extract_generic_enumerate_validator(
                field, attribute, meta_list,
            ))
        }
        v => {
            abort!(path.span(), "Unexpected list validator: {:?}", v)
        }
    }
}
