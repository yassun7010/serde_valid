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
    Ok(inner_extract_numeric_multiple_of_validator(
        field,
        validation_value,
        message_fn,
        rename_map,
    )?)
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
        if let Err(__composited_error_params) = ::serde_valid::validation::ValidateCompositedMultipleOf::validate_composited_multiple_of(
            #field_ident,
            #multiple_of,
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            match __composited_error_params {
                ::serde_valid::validation::Composited::Single(__single_error_params) => {
                    __properties_errors
                        .entry(#rename)
                        .or_default()
                        .push(::serde_valid::validation::Error::MultipleOf(
                            ::serde_valid::error::Message::new(
                                __single_error_params,
                                #message
                            )
                        ));
                    },
                ::serde_valid::validation::Composited::Array(__vec_error_params) => __vec_error_params
                    .into_iter()
                    .for_each(|__error_params| {
                        match __error_params {
                            ::serde_valid::validation::Composited::Single(__single_error_params) =>
                                __properties_errors
                                    .entry(#rename)
                                    .or_default()
                                    .push(::serde_valid::validation::Error::MultipleOf(
                                        ::serde_valid::error::Message::new(
                                            __single_error_params,
                                            #message
                                        )
                                    )),
                            _ => (),
                        }
                    }),
            }
        }
    ))
}
