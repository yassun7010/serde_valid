use crate::validate::number::extract_multiples_validator;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_validator_from_name_value(
    field_ident: &syn::Ident,
    _attribute: &syn::Attribute,
    syn::MetaNameValue { path, lit, .. }: &syn::MetaNameValue,
) -> Option<proc_macro2::TokenStream> {
    let ident = path.get_ident().unwrap();
    match ident.to_string().as_ref() {
        "multiple_of" => return Some(extract_multiples_validator(field_ident, lit)),
        v => {
            abort!(path.span(), "unexpected name value validator: {:?}", v)
        }
    }
}
