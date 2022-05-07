use proc_macro2::TokenStream;

use crate::types::{Field, SingleIdentPath};
use crate::validate::array::extract_array_unique_items_validator;
use crate::validate::common::{MetaListValidation, MetaNameValueValidation, MetaPathValidation};
use crate::validate::Validator;
use std::str::FromStr;

pub fn extract_validator_from_nested_meta_path(
    field: &impl Field,
    validation: &syn::Path,
    message_fn: Option<TokenStream>,
) -> Result<Validator, crate::Error> {
    let validation_ident = SingleIdentPath::new(validation).ident();
    let validation_name = validation_ident.to_string();
    match MetaPathValidation::from_str(&validation_name) {
        Ok(MetaPathValidation::UniqueItems) => {
            Ok(extract_array_unique_items_validator(field, message_fn))
        }
        Err(unknown) => {
            if MetaNameValueValidation::from_str(&validation_name).is_ok() {
                Err(crate::Error::validate_meta_name_value_need_value(
                    validation_ident.span(),
                    &validation_name,
                ))
            } else if MetaListValidation::from_str(&validation_name).is_ok() {
                Err(crate::Error::new_meta_list_need_value_error(
                    validation_ident.span(),
                    &validation_name,
                ))
            } else {
                Err(crate::Error::new_unknown_meta_error(
                    validation_ident.span(),
                    &unknown,
                    &(MetaPathValidation::iter().map(|x| x.name()))
                        .chain(MetaNameValueValidation::iter().map(|x| x.name()))
                        .chain(MetaListValidation::iter().map(|x| x.name()))
                        .collect::<Vec<_>>(),
                ))
            }
        }
    }
}
