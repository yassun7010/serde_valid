use crate::attribute::field_validate::common::{get_numeric, CustomMessageToken};
use crate::attribute::field_validate::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_numeric_multiple_of_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    inner_extract_numeric_multiple_of_validator(field, validation_value, custom_message, rename_map)
}

fn inner_extract_numeric_multiple_of_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();
    let multiple_of = get_numeric(validation_value)?;
    let custom_message = custom_message.into_token();

    Ok(quote!(
        if let Err(__composited_error_params) = ::serde_valid::validation::ValidateCompositedMultipleOf::validate_composited_multiple_of(
            #field_ident,
            #multiple_of,
        ) {
            use ::serde_valid::validation::{IntoError, ToDefaultMessage};

            #errors
                .entry(#rename)
                .or_default()
                .push(__composited_error_params.into_error_by(#custom_message.unwrap_or_default()));
        }
    ))
}
