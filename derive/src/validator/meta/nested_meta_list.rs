use crate::types::{Field, SingleIdentPath};
use crate::validator::common::MetaListValidation;
use crate::validator::generic::{
    extract_generic_custom_validator, extract_generic_enumerate_validator,
};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use std::str::FromStr;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
    message_fn: Option<TokenStream>,
) -> Result<Validator, crate::Error> {
    let syn::MetaList {
        path: validation_name,
        ..
    } = validation_list;
    let validation_ident = SingleIdentPath::new(&validation_name).ident();

    match MetaListValidation::from_str(&validation_ident.to_string()) {
        Ok(MetaListValidation::Enumerate) => {
            return Ok(extract_generic_enumerate_validator(
                field,
                attribute,
                validation_list,
                message_fn,
            )?)
        }
        Ok(MetaListValidation::Custom) => {
            return Ok(extract_generic_custom_validator(
                field,
                attribute,
                validation_list,
            ))
        }
        Err(unknown) => Err(crate::Error::new_unknown_meta_error(
            validation_name.span(),
            &unknown,
            &MetaListValidation::iter()
                .map(|x| x.name())
                .collect::<Vec<_>>(),
        )),
    }
}
