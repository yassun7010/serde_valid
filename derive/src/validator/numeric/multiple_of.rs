use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;

const VALIDATION_LABEL: &'static str = "multiple_of";

use crate::validator::common::get_numeric;
use crate::validator::Validator;

pub fn extract_numeric_multiple_of_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_numeric_multiple_of_validator(
            &array_field,
            validation_value,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_numeric_multiple_of_validator(
            &option_field,
            validation_value,
        )))
    } else {
        Validator::Normal(inner_extract_numeric_multiple_of_validator(
            field,
            validation_value,
        ))
    }
}

fn inner_extract_numeric_multiple_of_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
) -> TokenStream {
    let multiple_of = get_numeric(VALIDATION_LABEL, field, validation_value);
    let message = quote!(::serde_valid::MultipleOfParams::to_default_message);
    let field_name = field.name();
    let field_ident = field.ident();

    quote!(
        if !::serde_valid::validate_numeric_multiple_of(
            *#field_ident,
            #multiple_of,
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            __errors
                .entry(#field_name)
                .or_default()
                .push(
                    ::serde_valid::validation::Error::MultipleOf(
                        ::serde_valid::error::Message::new(
                            ::serde_valid::MultipleOfParams::new(
                                *#field_ident,
                                #multiple_of,
                            ),
                            #message
                        )
                    )
                );
        }
    )
}
