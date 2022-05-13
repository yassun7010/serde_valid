use std::collections::HashMap;

use crate::{types::Field, validate::Validator};
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_array_unique_items_validator(
    field: &impl Field,
    message_fn: Option<TokenStream>,
    rename_map: &HashMap<String, String>,
) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_array_unique_items_validator(
            &option_field,
            message_fn,
            rename_map,
        )))
    } else {
        Validator::Normal(inner_extract_array_unique_items_validator(
            field, message_fn, rename_map,
        ))
    }
}

fn inner_extract_array_unique_items_validator(
    field: &impl Field,
    message_fn: Option<TokenStream>,
    rename_map: &HashMap<String, String>,
) -> TokenStream {
    let field_name = field.name();
    let field_ident = field.ident();
    let rename = rename_map.get(field_name).unwrap_or(field_name);
    let message = message_fn.unwrap_or(quote!(
        ::serde_valid::UniqueItemsErrorParams::to_default_message
    ));

    quote!(
        if let Err(error_params) = ::serde_valid::ValidateUniqueItems::validate_unique_items(
            #field_ident
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            __errors
                .entry(#rename)
                .or_default()
                .push(::serde_valid::validation::Error::UniqueItems(
                    ::serde_valid::error::Message::new(
                        error_params,
                        #message
                    )
                ));
        }
    )
}
