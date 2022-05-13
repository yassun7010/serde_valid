use std::collections::HashMap;

use crate::types::Field;
use crate::validate::common::get_numeric;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_numeric_multiple_of_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    message_fn: Option<TokenStream>,
    rename_map: &HashMap<String, String>,
) -> Result<Validator, crate::Errors> {
    if let Some(array_field) = field.array_field() {
        Ok(Validator::Array(Box::new(
            extract_numeric_multiple_of_validator(
                &array_field,
                validation_value,
                message_fn,
                rename_map,
            )?,
        )))
    } else if let Some(option_field) = field.option_field() {
        Ok(Validator::Option(Box::new(
            extract_numeric_multiple_of_validator(
                &option_field,
                validation_value,
                message_fn,
                rename_map,
            )?,
        )))
    } else {
        Ok(Validator::Normal(
            inner_extract_numeric_multiple_of_validator(
                field,
                validation_value,
                message_fn,
                rename_map,
            )?,
        ))
    }
}

fn inner_extract_numeric_multiple_of_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    message_fn: Option<TokenStream>,
    rename_map: &HashMap<String, String>,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let rename = rename_map.get(field_name).unwrap_or(field_name);
    let multiple_of = get_numeric(validation_value)?;
    let message = message_fn.unwrap_or(quote!(
        ::serde_valid::MultipleOfErrorParams::to_default_message
    ));

    Ok(quote!(
        if let Err(error_params) = ::serde_valid::ValidateMultipleOf::validate_multiple_of(
            #field_ident,
            #multiple_of,
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            __errors
                .entry(#rename)
                .or_default()
                .push(
                    ::serde_valid::validation::Error::MultipleOf(
                        ::serde_valid::error::Message::new(
                            error_params,
                            #message
                        )
                    )
                );
        }
    ))
}
