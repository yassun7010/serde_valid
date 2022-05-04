use crate::types::Field;
use crate::validator::common::get_numeric;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
macro_rules! extract_numeric_range_validator{
    (
        $Params:tt,
        $ErrorType:tt,
        $field:tt,
        $label:tt,
        $function_name:ident,
        $inner_function_name:ident,
        $validate_function:ident
    ) => {
        pub fn $function_name(
            field: &impl Field,
            validation_value: &syn::Lit,
        ) -> Validator {
            if let Some(array_field) = field.array_field() {
                Validator::Array(Box::new($function_name(
                    &array_field,
                    validation_value,
                )))
            } else if let Some(option_field) = field.option_field() {
                Validator::Option(Box::new($function_name(
                    &option_field,
                    validation_value,
                )))
            } else {
                Validator::Normal($inner_function_name(
                    field,
                    validation_value,
                ))
            }
        }

        fn $inner_function_name(
            field: &impl Field,
            validation_value: &syn::Lit,
        ) -> TokenStream {
            let $field = get_numeric($label, field, validation_value);

            let field_name = field.name();
            let field_ident = field.ident();
            let message = quote!(::serde_valid::$Params::to_default_message);

            quote!(
                if !::serde_valid::$validate_function(
                    *#field_ident,
                    #$field,
                ) {
                    use ::serde_valid::error::ToDefaultMessage;
                    __errors
                        .entry(#field_name)
                        .or_default()
                        .push(::serde_valid::validation::Error::$ErrorType(
                            ::serde_valid::error::Message::new(
                                ::serde_valid::$Params::new(
                                    *#field_ident,
                                    #$field,
                                ),
                                #message
                            )
                        ));
                }
            )
        }
    }
}

extract_numeric_range_validator!(
    MinimumParams,
    Minimum,
    minimum,
    "minimum",
    extract_numeric_minimum_validator,
    inner_extract_numeric_minimum_validator,
    validate_numeric_minimum
);

extract_numeric_range_validator!(
    MaximumParams,
    Maximum,
    maximum,
    "maximum",
    extract_numeric_maximum_validator,
    inner_extract_numeric_maximum_validator,
    validate_numeric_maximum
);

extract_numeric_range_validator!(
    ExclusiveMinimumParams,
    ExclusiveMinimum,
    exclusive_minimum,
    "exclusive_minimum",
    extract_numeric_exclusive_minimum_validator,
    inner_extract_numeric_exclusive_minimum_validator,
    validate_numeric_exclusive_minimum
);

extract_numeric_range_validator!(
    ExclusiveMaximumParams,
    ExclusiveMaximum,
    exclusive_maximum,
    "exclusive_maximum",
    extract_numeric_exclusive_maximum_validator,
    inner_extract_numeric_exclusive_maximum_validator,
    validate_numeric_exclusive_maximum
);
