use crate::types::Field;
use crate::validate::common::get_numeric;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
macro_rules! extract_string_length_validator{
    (
        $ErrorParams:tt,
        $ErrorType:tt,
        $field:tt,
        $function_name:ident,
        $inner_function_name:ident,
        $ValidateTrait:ident,
        $validation_method:ident
    ) => {
        pub fn $function_name(
            field: &impl Field,
            validation_value: &syn::Lit,
            message_fn: Option<TokenStream>,
            rename_map: &HashMap<String, String>,
        ) -> Result<Validator, crate::Errors> {
            Ok($inner_function_name(field, validation_value, message_fn, rename_map)?)
        }

        fn $inner_function_name(
            field: &impl Field,
            validation_value: &syn::Lit,
            message_fn: Option<TokenStream>,
            rename_map: &HashMap<String, String>,
        ) -> Result<TokenStream, crate::Errors> {
            let field_name = field.name();
            let field_ident = field.ident();
            let rename = rename_map.get(field_name).unwrap_or(field_name);
            let $field = get_numeric(validation_value)?;
            let message =
                message_fn.unwrap_or(quote!(::serde_valid::$ErrorParams::to_default_message));

            Ok(quote!(
                // if let Err(error_params) = ::serde_valid::$ValidateTrait::$validation_method(
                //     #field_ident,
                //     #$field,
                // ) {
                //     use ::serde_valid::error::ToDefaultMessage;
                //     __properties_errors
                //         .entry(#rename)
                //         .or_default()
                //         .push(::serde_valid::validation::Error::$ErrorType(
                //             ::serde_valid::error::Message::new(
                //                 error_params,
                //                 #message
                //             )
                //         ));
                // }
            ))
        }
    }
}

extract_string_length_validator!(
    MaxLengthErrorParams,
    MaxLength,
    max_length,
    extract_string_max_length_validator,
    inner_extract_string_max_length_validator,
    ValidateMaxLength,
    validate_max_length
);
extract_string_length_validator!(
    MinLengthErrorParams,
    MinLength,
    min_length,
    extract_string_min_length_validator,
    inner_extract_string_min_length_validator,
    ValidateMinLength,
    validate_min_length
);
