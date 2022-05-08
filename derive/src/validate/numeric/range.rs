use crate::types::Field;
use crate::validate::common::get_numeric;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

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
        $validate_function:ident
    ) => {
        pub fn $function_name(
            field: &impl Field,
            validation_value: &syn::Lit,
            message_fn: Option<TokenStream>,
        ) -> Result<Validator, crate::Errors> {
            if let Some(array_field) = field.array_field() {
                Ok(Validator::Array(Box::new(
                    $function_name(&array_field, validation_value, message_fn)?
                )))
            } else if let Some(option_field) = field.option_field() {
                Ok(Validator::Option(Box::new(
                    $function_name(&option_field, validation_value, message_fn)?
                )))
            } else {
                Ok(Validator::Normal(
                    $inner_function_name(field, validation_value, message_fn)?
                ))
            }
        }

        fn $inner_function_name(
            field: &impl Field,
            validation_value: &syn::Lit,
            message_fn: Option<TokenStream>,
        ) -> Result<TokenStream, crate::Errors> {
            let field_name = field.name();
            let field_ident = field.ident();
            let $limit = get_numeric(validation_value)?;
            let message =
                message_fn.unwrap_or(quote!(::serde_valid::$ErrorParams::to_default_message));

            Ok(quote!(
                if !::serde_valid::$validate_function(
                    *#field_ident,
                    #$limit,
                ) {
                    use ::serde_valid::error::ToDefaultMessage;
                    __errors
                        .entry(#field_name)
                        .or_default()
                        .push(::serde_valid::validation::Error::$ErrorType(
                            ::serde_valid::error::Message::new(
                                ::serde_valid::$ErrorParams::new(
                                    *#field_ident,
                                    #$limit,
                                ),
                                #message
                            )
                        ));
                }
            ))
        }
    }
}

extract_numeric_range_validator!(
    MinimumErrorParams,
    Minimum,
    minimum,
    extract_numeric_minimum_validator,
    inner_extract_numeric_minimum_validator,
    validate_numeric_minimum
);

extract_numeric_range_validator!(
    MaximumErrorParams,
    Maximum,
    maximum,
    extract_numeric_maximum_validator,
    inner_extract_numeric_maximum_validator,
    validate_numeric_maximum
);

extract_numeric_range_validator!(
    ExclusiveMinimumErrorParams,
    ExclusiveMinimum,
    exclusive_minimum,
    extract_numeric_exclusive_minimum_validator,
    inner_extract_numeric_exclusive_minimum_validator,
    validate_numeric_exclusive_minimum
);

extract_numeric_range_validator!(
    ExclusiveMaximumErrorParams,
    ExclusiveMaximum,
    exclusive_maximum,
    extract_numeric_exclusive_maximum_validator,
    inner_extract_numeric_exclusive_maximum_validator,
    validate_numeric_exclusive_maximum
);
