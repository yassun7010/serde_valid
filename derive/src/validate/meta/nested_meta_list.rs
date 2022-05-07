use crate::types::{Field, SingleIdentPath};
use crate::validate::common::MetaListValidation;
use crate::validate::generic::{
    extract_generic_custom_validator, extract_generic_enumerate_validator,
};
use crate::validate::Validator;
use proc_macro2::TokenStream;
use std::str::FromStr;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
    message_fn: Option<TokenStream>,
) -> Result<Validator, crate::Errors> {
    let syn::MetaList {
        path: validation_name,
        ..
    } = validation_list;
    let validation_ident = SingleIdentPath::new(&validation_name).ident();

    match MetaListValidation::from_str(&validation_ident.to_string()) {
        Ok(MetaListValidation::Enumerate) => {
            extract_generic_enumerate_validator(field, attribute, validation_list, message_fn)
        }
        Ok(MetaListValidation::Custom) => extract_generic_custom_validator(field, validation_list),
        Err(unknown) => Err(vec![crate::Error::validate_unknown_type(
            validation_name.span(),
            &unknown,
            &MetaListValidation::iter()
                .map(|x| x.name())
                .collect::<Vec<_>>(),
        )]),
    }
}
