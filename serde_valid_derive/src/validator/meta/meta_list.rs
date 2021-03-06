use crate::validator::number::extract_range_validator;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_validator_from_meta_list(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    syn::MetaList { path, nested, .. }: &syn::MetaList,
) -> Option<proc_macro2::TokenStream> {
    let ident = path.get_ident().unwrap();

    match ident.to_string().as_ref() {
        "range" => return Some(extract_range_validator(field_ident, attribute, nested)),
        v => {
            abort!(path.span(), "unexpected list validator: {:?}", v)
        }
    }
}
