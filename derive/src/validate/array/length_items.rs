use crate::types::Field;
use crate::validate::common::get_numeric;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
macro_rules! extract_array_length_validator{
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
                if let Err(error_params) = ::serde_valid::$ValidateTrait::$validation_method(
                    #field_ident,
                    #$limit,
                ) {
                    use ::serde_valid::error::ToDefaultMessage;
                    __errors
                        .entry(#rename)
                        .or_default()
                        .push(::serde_valid::validation::Error::$ErrorType(
                            ::serde_valid::error::Message::new(
                                error_params,
                                #message
                            )
                        ));
                }
            ))
        }
    }
}

extract_array_length_validator!(
    MaxItemsErrorParams,
    MaxItems,
    max_items,
    extract_array_max_items_validator,
    inner_extract_array_max_items_validator,
    ValidateMaxItems,
    validate_max_items
);

extract_array_length_validator!(
    MinItemsErrorParams,
    MinItems,
    min_items,
    extract_array_min_items_validator,
    inner_extract_array_min_items_validator,
    ValidateMinItems,
    validate_min_items
);
