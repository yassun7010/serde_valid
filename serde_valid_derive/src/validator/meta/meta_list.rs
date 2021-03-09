use crate::helper::NamedField;
use crate::validator::number::extract_range_validator;
use crate::validator::Validator;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_validator_from_meta_list(
    field: &NamedField,
    attribute: &syn::Attribute,
    syn::MetaList { path, nested, .. }: &syn::MetaList,
) -> Option<Validator> {
    let ident = path.get_ident().unwrap();

    match ident.to_string().as_ref() {
        "range" => return Some(extract_range_validator(field, attribute, nested)),
        v => {
            abort!(path.span(), "unexpected list validator: {:?}", v)
        }
    }
}
