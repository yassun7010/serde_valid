use crate::serde::rename::RenameMap;
use crate::types::Field;
use crate::validate::common::{get_numeric, CustomMessage};
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_numeric_multiple_of_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    custom_message: CustomMessage,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    inner_extract_numeric_multiple_of_validator(field, validation_value, custom_message, rename_map)
}

fn inner_extract_numeric_multiple_of_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    custom_message: CustomMessage,
    rename_map: &RenameMap,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();
    let multiple_of = get_numeric(validation_value)?;
    let message_fn = custom_message
        .message_fn
        .unwrap_or(quote!(::serde_valid::MultipleOfError::to_default_message));

    Ok(quote!(
        if let Err(__composited_error_params) = ::serde_valid::validation::ValidateCompositedMultipleOf::validate_composited_multiple_of(
            #field_ident,
            #multiple_of,
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            use ::serde_valid::validation::IntoError;

            #errors
                .entry(#rename)
                .or_default()
                .push(__composited_error_params.into_error_by(#message_fn)
            );
        }
    ))
}
