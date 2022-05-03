use crate::types::{Field, SingleIdentPath};
use crate::validator::array::extract_array_unique_items_validator_from_meta_path;
use crate::validator::Validator;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_path(
    field: &impl Field,
    _attribute: &syn::Attribute,
    validation: &syn::Path,
) -> Result<Validator, Vec<syn::Error>> {
    let validation_ident = SingleIdentPath::new(validation).ident();
    match validation_ident.to_string().as_ref() {
        "unique_items" => return Ok(extract_array_unique_items_validator_from_meta_path(field)),
        v => Err(vec![syn::Error::new(
            validation.span(),
            format!("Unexpected name value validator: {v:?}"),
        )]),
    }
}
