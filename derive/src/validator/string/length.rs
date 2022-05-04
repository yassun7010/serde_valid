use crate::types::Field;
use crate::validator::common::get_numeric;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
macro_rules! extract_string_length_validator{
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
                match array_field.ty() {
                    syn::Type::Path(element_type) => {
                        if let Some(element_type_ident) = element_type.path.get_ident() {
                            if ["u8", "char"].contains(&element_type_ident.to_string().as_str()) {
                                return Validator::Normal($inner_function_name(
                                    field,
                                    validation_value,
                                ));
                            }
                        }
                    }
                    _ => (),
                }
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
            let field_name = field.name();
            let field_ident = field.ident();
            let $field = get_numeric($label, field, validation_value);
            let message = quote!(::serde_valid::$Params::to_default_message);

            quote!(
                if !::serde_valid::$validate_function(
                    #field_ident,
                    #$field,
                ) {
                    use ::serde_valid::error::ToDefaultMessage;
                    __errors
                        .entry(#field_name)
                        .or_default()
                        .push(::serde_valid::validation::Error::$ErrorType(
                            ::serde_valid::error::Message::new(
                                ::serde_valid::$Params::new(
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

extract_string_length_validator!(
    MaxLengthParams,
    MaxLength,
    max_length,
    "max_length",
    extract_string_max_length_validator,
    inner_extract_string_max_length_validator,
    validate_string_max_length
);
extract_string_length_validator!(
    MinLengthParams,
    MinLength,
    min_length,
    "min_length",
    extract_string_min_length_validator,
    inner_extract_string_min_length_validator,
    validate_string_min_length
);
