use crate::types::{Field, SingleIdentPath};
use crate::validate::array::{
    extract_array_max_items_validator, extract_array_min_items_validator,
};
use crate::validate::common::MetaNameValueValidation;
use crate::validate::numeric::{
    extract_numeric_exclusive_maximum_validator, extract_numeric_exclusive_minimum_validator,
    extract_numeric_maximum_validator, extract_numeric_minimum_validator,
    extract_numeric_multiple_of_validator,
};
use crate::validate::object::{
    extract_object_max_properties_validator, extract_object_min_properties_validator,
};
use crate::validate::string::{
    extract_string_max_length_validator, extract_string_min_length_validator,
    extract_string_pattern_validator,
};
use crate::validate::Validator;
use proc_macro2::TokenStream;
use std::str::FromStr;
use syn::spanned::Spanned;

pub fn extract_validator_from_nested_meta_name_value(
    field: &impl Field,
    _attribute: &syn::Attribute,
    syn::MetaNameValue {
        path: validation_name,
        lit: validation_value,
        ..
    }: &syn::MetaNameValue,
    message_fn: Option<TokenStream>,
) -> Result<Validator, crate::Error> {
    let validation_name_ident = SingleIdentPath::new(validation_name).ident();
    match MetaNameValueValidation::from_str(&validation_name_ident.to_string()) {
        Ok(MetaNameValueValidation::Minimum) => {
            return Ok(extract_numeric_minimum_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Ok(MetaNameValueValidation::Maximum) => {
            return Ok(extract_numeric_maximum_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Ok(MetaNameValueValidation::ExclusiveMinimum) => {
            return Ok(extract_numeric_exclusive_minimum_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Ok(MetaNameValueValidation::ExclusiveMaximum) => {
            return Ok(extract_numeric_exclusive_maximum_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Ok(MetaNameValueValidation::MinLength) => {
            return Ok(extract_string_min_length_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Ok(MetaNameValueValidation::MaxLength) => {
            return Ok(extract_string_max_length_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Ok(MetaNameValueValidation::MinItems) => {
            return Ok(extract_array_min_items_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Ok(MetaNameValueValidation::MaxItems) => {
            return Ok(extract_array_max_items_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Ok(MetaNameValueValidation::MinProperties) => {
            return Ok(extract_object_min_properties_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Ok(MetaNameValueValidation::MaxProperties) => {
            return Ok(extract_object_max_properties_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Ok(MetaNameValueValidation::MultipleOf) => {
            return Ok(extract_numeric_multiple_of_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Ok(MetaNameValueValidation::Pattern) => {
            return Ok(extract_string_pattern_validator(
                field,
                validation_value,
                message_fn,
            )?)
        }
        Err(unknown) => Err(crate::Error::new_unknown_meta_error(
            validation_name.span(),
            &unknown,
            &MetaNameValueValidation::iter()
                .map(|x| x.name())
                .collect::<Vec<_>>(),
        )),
    }
}
