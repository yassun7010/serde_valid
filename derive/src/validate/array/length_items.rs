use crate::types::Field;
use crate::validate::common::get_numeric;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

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
        $validate_function:ident
    ) => {
        pub fn $function_name(
            field: &impl Field,
            validation_value: &syn::Lit,
            message_fn: Option<TokenStream>,
        ) -> Result<Validator, crate::Error> {
            if let Some(option_field) = field.option_field() {
                Ok(Validator::Option(Box::new(
                    $function_name(&option_field, validation_value, message_fn)?
                )))
            } else {
                Ok(Validator::Normal($inner_function_name(field, validation_value, message_fn)?))
            }
        }

        fn $inner_function_name(
            field: &impl Field,
            validation_value: &syn::Lit,
            message_fn: Option<TokenStream>,
        ) -> Result<TokenStream, crate::Error> {
            let field_name = field.name();
            let field_ident = field.ident();
            let $limit = get_numeric(validation_value)?;
            let message =
                message_fn.unwrap_or(quote!(::serde_valid::$ErrorParams::to_default_message));

            Ok(quote!(
                if !::serde_valid::$validate_function(
                    #field_ident,
                    #$limit,
                ) {
                    use ::serde_valid::error::ToDefaultMessage;
                    __errors
                        .entry(#field_name)
                        .or_default()
                        .push(::serde_valid::validation::Error::$ErrorType(
                            ::serde_valid::error::Message::new(
                                ::serde_valid::$ErrorParams::new(
                                    #field_ident,
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

extract_array_length_validator!(
    MaxItemsErrorParams,
    MaxItems,
    max_items,
    extract_array_max_items_validator,
    inner_extract_array_max_items_validator,
    validate_array_max_items
);

extract_array_length_validator!(
    MinItemsErrorParams,
    MinItems,
    min_items,
    extract_array_min_items_validator,
    inner_extract_array_min_items_validator,
    validate_array_min_items
);
