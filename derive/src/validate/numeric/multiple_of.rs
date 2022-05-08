use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;

use crate::validate::common::get_numeric;
use crate::validate::Validator;

pub fn extract_numeric_multiple_of_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    message_fn: Option<TokenStream>,
) -> Result<Validator, crate::Errors> {
    if let Some(array_field) = field.array_field() {
        Ok(Validator::Array(Box::new(
            extract_numeric_multiple_of_validator(&array_field, validation_value, message_fn)?,
        )))
    } else if let Some(option_field) = field.option_field() {
        Ok(Validator::Option(Box::new(
            extract_numeric_multiple_of_validator(&option_field, validation_value, message_fn)?,
        )))
    } else {
        Ok(Validator::Normal(
            inner_extract_numeric_multiple_of_validator(field, validation_value, message_fn)?,
        ))
    }
}

fn inner_extract_numeric_multiple_of_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
    message_fn: Option<TokenStream>,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let multiple_of = get_numeric(validation_value)?;
    let message = message_fn.unwrap_or(quote!(
        ::serde_valid::MultipleOfErrorParams::to_default_message
    ));

    Ok(quote!(
        if !::serde_valid::ValidateNumericMultipleOf::validate(
            #field_ident,
            #multiple_of,
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            __errors
                .entry(#field_name)
                .or_default()
                .push(
                    ::serde_valid::validation::Error::MultipleOf(
                        ::serde_valid::error::Message::new(
                            ::serde_valid::MultipleOfErrorParams::new(
                                *#field_ident,
                                #multiple_of,
                            ),
                            #message
                        )
                    )
                );
        }
    ))
}
