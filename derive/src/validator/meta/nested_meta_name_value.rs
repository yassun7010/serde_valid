use crate::types::{NamedField, SingleIdentPath};
use crate::validator::numeric::extract_numeric_multiple_of_validator_from_meta_name_value;
use crate::validator::string::extract_string_pattern_validator_from_meta_name_value;
use crate::validator::Validator;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_name_value(
    field: &NamedField,
    _attribute: &syn::Attribute,
    syn::MetaNameValue { path, lit, .. }: &syn::MetaNameValue,
) -> Option<Validator> {
    let path_ident = SingleIdentPath::new(path).ident();
    match path_ident.to_string().as_ref() {
        "multiple_of" => {
            return Some(extract_numeric_multiple_of_validator_from_meta_name_value(
                field, lit,
            ))
        }
        "pattern" => {
            return Some(extract_string_pattern_validator_from_meta_name_value(
                field, lit,
            ))
        }
        v => {
            abort!(path.span(), "Unexpected name value validator: {:?}", v)
        }
    }
}
