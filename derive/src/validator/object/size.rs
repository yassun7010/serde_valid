use crate::types::Field;
use crate::validator::common::get_numeric;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/object.html#size>
macro_rules! extract_object_size_validator{
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
            let message = quote!(::serde_valid::validation::$Params::to_default_message);

            quote!(
                if !::serde_valid::$validate_function(
                    #field_ident,
                    #$field
                ) {
                    use ::serde_valid::error::ToDefaultMessage;
                    __errors
                        .entry(#field_name)
                        .or_default()
                        .push(::serde_valid::validation::Error::$ErrorType(
                            ::serde_valid::error::Message::new(
                                ::serde_valid::validation::$Params::new(
                                    #field_ident,
                                    #$field
                                ),
                                #message
                            )
                        ));
                }
            )
        }
    }
}

extract_object_size_validator!(
    MaxPropertiesParams,
    MaxProperties,
    max_properties,
    "max_properties",
    extract_object_max_properties_validator,
    inner_extract_object_max_properties_validator,
    validate_object_max_properties
);

extract_object_size_validator!(
    MinPropertiesParams,
    MinProperties,
    min_properties,
    "min_properties",
    extract_object_min_properties_validator,
    inner_extract_object_min_properties_validator,
    validate_object_min_properties
);
