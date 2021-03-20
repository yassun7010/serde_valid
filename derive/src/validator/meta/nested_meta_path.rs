use crate::helper::{NamedField, SingleIdentPath};
use crate::validator::array::extract_array_uniqueness_validator;
use crate::validator::Validator;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_path(
    field: &NamedField,
    _attribute: &syn::Attribute,
    path: &syn::Path,
) -> Option<Validator> {
    let path_ident = SingleIdentPath::new(path).ident();
    match path_ident.to_string().as_ref() {
        "unique_items" => return Some(extract_array_uniqueness_validator(field)),
        v => {
            abort!(path.span(), "Unexpected name value validator: {:?}", v)
        }
    }
}
