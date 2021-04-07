use crate::types::{Field, SingleIdentPath};
use crate::validator::array::extract_array_unique_items_validator_from_meta_path;
use crate::validator::Validator;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_path<F: Field>(
    field: &F,
    _attribute: &syn::Attribute,
    validation: &syn::Path,
) -> Option<Validator> {
    let validation_ident = SingleIdentPath::new(validation).ident();
    match validation_ident.to_string().as_ref() {
        "unique_items" => return Some(extract_array_unique_items_validator_from_meta_path(field)),
        v => {
            abort!(
                validation.span(),
                "Unexpected name value validator: {:?}",
                v
            )
        }
    }
}
