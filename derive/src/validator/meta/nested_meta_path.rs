use crate::errors::Error;
use crate::types::{Field, SingleIdentPath};
use crate::validator::array::extract_array_unique_items_validator;
use crate::validator::common::{MetaListValidation, MetaNameValueValidation, MetaPathValidation};
use crate::validator::Validator;
use std::str::FromStr;

pub fn extract_validator_from_nested_meta_path(
    field: &impl Field,
    _attribute: &syn::Attribute,
    validation: &syn::Path,
) -> Result<Validator, Error> {
    let validation_ident = SingleIdentPath::new(validation).ident();
    match MetaPathValidation::from_str(&validation_ident.to_string()) {
        Ok(MetaPathValidation::UniqueItems) => Ok(extract_array_unique_items_validator(field)),
        Err(unknown) => Err(Error::new_unknown_meta_error(
            validation_ident.span(),
            &unknown,
            &MetaPathValidation::iter()
                .map(|x| x.name())
                .chain(MetaNameValueValidation::iter().map(|x| x.name()))
                .chain(MetaListValidation::iter().map(|x| x.name()))
                .collect::<Vec<_>>(),
        )),
    }
}
