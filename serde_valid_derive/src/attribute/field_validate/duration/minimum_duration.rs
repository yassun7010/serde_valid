use proc_macro2::TokenStream;
use quote::quote;
use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::field_validate::duration::extract_duration;
use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;

pub fn extract_string_minimum_duration_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    inner_extract_string_duration_validator(field, validation_value, message_format, rename_map)
}

fn inner_extract_string_duration_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();
    let duration = extract_duration(&validation_value)?;

    Ok(quote!(
        #duration
        if let Err(__composited_error_params) = ::serde_valid::validation::ValidateCompositedMinimumDuration::validate_composited_duration(
            #field_ident,
            duration,
        ) {
            use ::serde_valid::validation::IntoError;
            use ::serde_valid::validation::error::FormatDefault;

            #errors
                .entry(#rename)
                .or_default()
                .push(__composited_error_params.into_error_by(#message_format));
        }
    ))
}
