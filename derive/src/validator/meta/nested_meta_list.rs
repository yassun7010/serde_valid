use crate::helper::{NamedField, SingleIdentPath};
use crate::validator::array::extract_array_length_validator;
use crate::validator::generic::extract_generic_enumerate_validator;
use crate::validator::numeric::extract_numeric_range_validator;
use crate::validator::object::extract_object_size_validator;
use crate::validator::string::extract_string_length_validator;
use crate::validator::Validator;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_list(
    field: &NamedField,
    attribute: &syn::Attribute,
    syn::MetaList { path, nested, .. }: &syn::MetaList,
) -> Option<Validator> {
    let ident = SingleIdentPath::new(&path).ident();

    match ident.to_string().as_ref() {
        "range" => return Some(extract_numeric_range_validator(field, attribute, nested)),
        "length" => return Some(extract_string_length_validator(field, attribute, nested)),
        "items" => return Some(extract_array_length_validator(field, attribute, nested)),
        "properties" => return Some(extract_object_size_validator(field, attribute, nested)),
        "enumerate" => {
            return Some(extract_generic_enumerate_validator(
                field, attribute, nested,
            ))
        }
        v => {
            abort!(path.span(), "Unexpected list validator: {:?}", v)
        }
    }
}
