use crate::types::{Field, SingleIdentPath};
use crate::validator::numeric::extract_numeric_multiple_of_validator_from_meta_name_value;
use crate::validator::string::extract_string_pattern_validator_from_meta_name_value;
use crate::validator::Validator;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_name_value(
    field: &impl Field,
    _attribute: &syn::Attribute,
    syn::MetaNameValue {
        path: validation_name,
        lit: validation_value,
        ..
    }: &syn::MetaNameValue,
) -> Option<Validator> {
    let validation_name_ident = SingleIdentPath::new(validation_name).ident();
    match validation_name_ident.to_string().as_ref() {
        "multiple_of" => {
            return Some(extract_numeric_multiple_of_validator_from_meta_name_value(
                field,
                validation_value,
            ))
        }
        "pattern" => {
            return Some(extract_string_pattern_validator_from_meta_name_value(
                field,
                validation_value,
            ))
        }
        v => {
            abort!(
                validation_name.span(),
                "Unexpected name value validator: {:?}",
                v
            )
        }
    }
}
