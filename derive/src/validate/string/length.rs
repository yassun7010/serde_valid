use crate::types::Field;
use crate::validate::common::get_numeric;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

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
        $ValidateTrait:ident
    ) => {
        pub fn $function_name(
            field: &impl Field,
            validation_value: &syn::Lit,
            message_fn: Option<TokenStream>,
        ) -> Result<Validator, crate::Errors> {
            if let Some(array_field) = field.array_field() {
                match array_field.ty() {
                    syn::Type::Path(element_type) => {
                        if let Some(element_type_ident) = element_type.path.get_ident() {
                            if ["u8", "char"].contains(&element_type_ident.to_string().as_str()) {
                                return Ok(Validator::Normal(
                                    $inner_function_name(field, validation_value, message_fn)?
                                ));
                            }
                        }
                    }
                    _ => (),
                }
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
            let $field = get_numeric(validation_value)?;
            let message =
                message_fn.unwrap_or(quote!(::serde_valid::$ErrorParams::to_default_message));

            Ok(quote!(
                if !::serde_valid::$ValidateTrait::check(
                    #field_ident,
                    #$field,
                ) {
                    use ::serde_valid::error::ToDefaultMessage;
                    __errors
                        .entry(#field_name)
                        .or_default()
                        .push(::serde_valid::validation::Error::$ErrorType(
                            ::serde_valid::error::Message::new(
                                ::serde_valid::$ErrorParams::new(
                                    #field_ident,
                                    #$field,
                                ),
                                #message
                            )
                        ));
                }
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
    ValidateStringMaxLength
);
extract_string_length_validator!(
    MinLengthErrorParams,
    MinLength,
    min_length,
    extract_string_min_length_validator,
    inner_extract_string_min_length_validator,
    ValidateStringMinLength
);
