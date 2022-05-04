use crate::types::Field;
use crate::validator::common::get_numeric;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
macro_rules! extract_array_length_validator{
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
            if let Some(option_field) = field.option_field() {
                Validator::Option(Box::new($function_name(
                    &option_field,
                    validation_value,
                )))
            } else {
                Validator::Normal($inner_function_name(field, validation_value))
            }
        }

        fn $inner_function_name(
            field: &impl Field,
            validation_value: &syn::Lit,
        ) -> TokenStream {
            let $field = get_numeric($label, field, validation_value);

            let field_name = field.name();
            let field_ident = field.ident();
            let message = quote!(::serde_valid::validation::error::$Params::to_default_message);

            quote!(
                if !::serde_valid::$validate_function(
                    #field_ident,
                    #$field,
                ) {
                    use ::serde_valid::validation::error::ToDefaultMessage;
                    __errors
                        .entry(#field_name)
                        .or_default()
                        .push(::serde_valid::validation::Error::$ErrorType(
                            ::serde_valid::validation::error::Message::new(
                                ::serde_valid::validation::error::$Params::new(
                                    #field_ident,
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

extract_array_length_validator!(
    MaxItemsParams,
    MaxItems,
    max_items,
    "max_items",
    extract_array_max_items_validator,
    inner_extract_array_max_items_validator,
    validate_array_max_items
);

extract_array_length_validator!(
    MinItemsParams,
    MinItems,
    min_items,
    "min_items",
    extract_array_min_items_validator,
    inner_extract_array_min_items_validator,
    validate_array_min_items
);
