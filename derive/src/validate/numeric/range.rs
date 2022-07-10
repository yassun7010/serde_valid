use crate::types::Field;
use crate::validate::common::get_numeric;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
macro_rules! extract_numeric_range_validator{
    (
        $ErrorParams:tt,
        $ErrorType:tt,
        $limit:tt,
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
            let $limit = get_numeric(validation_value)?;
            let message =
                message_fn.unwrap_or(quote!(::serde_valid::$ErrorParams::to_default_message));

            Ok(quote!(
                if let Err(__composited_error_params) = ::serde_valid::validation::$ValidateTrait::$validation_method(
                    #field_ident,
                    #$limit,
                ) {
                    use ::serde_valid::error::ToDefaultMessage;
                    use ::serde_valid::validation::ConvertIntoError;

                    __properties_errors
                        .entry(#rename)
                        .or_default()
                        .push(__composited_error_params.convert_into_error(#message)
                    );
                }
            ))
        }
    }
}

extract_numeric_range_validator!(
    MaximumErrorParams,
    Maximum,
    maximum,
    extract_numeric_maximum_validator,
    inner_extract_numeric_maximum_validator,
    ValidateCompositedMaximum,
    validate_composited_maximum
);

extract_numeric_range_validator!(
    MinimumErrorParams,
    Minimum,
    minimum,
    extract_numeric_minimum_validator,
    inner_extract_numeric_minimum_validator,
    ValidateCompositedMinimum,
    validate_composited_minimum
);

extract_numeric_range_validator!(
    ExclusiveMaximumErrorParams,
    ExclusiveMaximum,
    exclusive_maximum,
    extract_numeric_exclusive_maximum_validator,
    inner_extract_numeric_exclusive_maximum_validator,
    ValidateCompositedExclusiveMaximum,
    validate_composited_exclusive_maximum
);

extract_numeric_range_validator!(
    ExclusiveMinimumErrorParams,
    ExclusiveMinimum,
    exclusive_minimum,
    extract_numeric_exclusive_minimum_validator,
    inner_extract_numeric_exclusive_minimum_validator,
    ValidateCompositedExclusiveMinimum,
    validate_composited_exclusive_minimum
);
