use crate::helper::NamedField;
use crate::validator::{number::extract_multiples_validator, Validator};
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_validator_from_name_value(
    field: &NamedField,
    _attribute: &syn::Attribute,
    syn::MetaNameValue { path, lit, .. }: &syn::MetaNameValue,
) -> Option<Validator> {
    let path_ident = path.get_ident().unwrap();
    match path_ident.to_string().as_ref() {
        "multiple_of" => return Some(extract_multiples_validator(field, lit)),
        v => {
            abort!(path.span(), "unexpected name value validator: {:?}", v)
        }
    }
}
