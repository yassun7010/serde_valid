use crate::errors::Error;
use crate::types::{Field, SingleIdentPath};
use crate::validator::array::extract_array_unique_items_validator_from_meta_path;
use crate::validator::Validator;

pub fn extract_validator_from_nested_meta_path(
    field: &impl Field,
    _attribute: &syn::Attribute,
    validation: &syn::Path,
) -> Result<Validator, Error> {
    let validation_ident = SingleIdentPath::new(validation).ident();
    match validation_ident.to_string().as_ref() {
        "unique_items" => return Ok(extract_array_unique_items_validator_from_meta_path(field)),
        target => Err(Error::new_path_meta_name_error(
            validation_ident.span(),
            target,
            &["unique_items"],
        )),
    }
}
