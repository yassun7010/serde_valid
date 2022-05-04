use crate::{types::Field, validator::Validator};
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_array_unique_items_validator(field: &impl Field) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_array_unique_items_validator(
            &option_field,
        )))
    } else {
        let message = quote!(::serde_valid::UniqueItemsParams::to_default_message);
        Validator::Normal(inner_extract_array_unique_items_validator(field, message))
    }
}

fn inner_extract_array_unique_items_validator(
    field: &impl Field,
    message: TokenStream,
) -> TokenStream {
    let field_name = field.name();
    let field_ident = field.ident();
    quote!(
        if !::serde_valid::validate_array_unique_items(
            #field_ident
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            __errors
                .entry(#field_name)
                .or_default()
                .push(::serde_valid::validation::Error::UniqueItems(
                    ::serde_valid::error::Message::new(
                        ::serde_valid::UniqueItemsParams::new(
                            #field_ident,
                        ),
                        #message
                    )
                ));
        }
    )
}
