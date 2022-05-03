use crate::types::Field;
use crate::validator::common::get_numeric;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

const VALIDATION_LABEL: &'static str = "min_items";

pub fn extract_array_min_items_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_array_min_items_validator(
            &option_field,
            validation_value,
        )))
    } else {
        Validator::Normal(inner_extract_array_items_validator(field, validation_value))
    }
}

fn inner_extract_array_items_validator(
    field: &impl Field,
    validation_value: &syn::Lit,
) -> TokenStream {
    let min_item = get_numeric(VALIDATION_LABEL, field, validation_value);

    let field_name = field.name();
    let field_ident = field.ident();
    let message = quote!(::serde_valid::validation::error::MinItemsParams::to_default_message);

    quote!(
        if !::serde_valid::validate_array_min_items(
            #field_ident,
            #min_item,
        ) {
            use ::serde_valid::validation::error::ToDefaultMessage;
            __errors
                .entry(#field_name)
                .or_default()
                .push(::serde_valid::validation::Error::MinItems(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::MinItemsParams::new(
                            #field_ident,
                            #min_item,
                        ),
                        #message
                    )
                ));
        }
    )
}
