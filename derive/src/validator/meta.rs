mod meta_list;
mod meta_path;
mod nested_meta_list;
mod nested_meta_name_value;
mod nested_meta_path;

use crate::types::Field;
use crate::validator::Validator;
use meta_path::extract_validator_from_meta_path;
use syn::spanned::Spanned;

use self::meta_list::extract_validator_from_meta_list;

pub fn extract_meta_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
) -> Result<Validator, crate::Error> {
    match attribute.parse_meta() {
        Ok(syn::Meta::List(list)) => extract_validator_from_meta_list(field, attribute, &list),
        Ok(syn::Meta::Path(_)) => extract_validator_from_meta_path(field),
        Ok(syn::Meta::NameValue(_)) => Err(crate::Error::new_meta_name_value_item_error(
            attribute.span(),
        )),
        Err(error) => Err(crate::Error::attribute_parse_error(attribute, &error)),
    }
}
