use crate::helper::NamedField;
use crate::validator::number::extract_number_multiples_validator;
use crate::validator::string::extract_string_pattern_validator;
use crate::validator::Validator;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_validator_from_name_value(
    field: &NamedField,
    _attribute: &syn::Attribute,
    syn::MetaNameValue { path, lit, .. }: &syn::MetaNameValue,
) -> Option<Validator> {
    let path_ident = path.get_ident().unwrap();
    match path_ident.to_string().as_ref() {
        "multiple_of" => return Some(extract_number_multiples_validator(field, lit)),
        "pattern" => return Some(extract_string_pattern_validator(field, lit)),
        v => {
            abort!(path.span(), "unexpected name value validator: {:?}", v)
        }
    }
}
