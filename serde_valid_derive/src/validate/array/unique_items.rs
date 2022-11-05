use crate::{
    serde::rename::RenameMap,
    types::Field,
    validate::{common::CustomMessageToken, Validator},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_array_unique_items_validator(
    field: &impl Field,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Validator {
    inner_extract_array_unique_items_validator(field, custom_message, rename_map)
}

fn inner_extract_array_unique_items_validator(
    field: &impl Field,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> TokenStream {
    let field_name = field.name();
    let field_ident = field.ident();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();
    let message_fn = custom_message
        .message_fn
        .unwrap_or(quote!(::serde_valid::UniqueItemsError::to_default_message));

    quote!(
        if let Err(error_params) = ::serde_valid::ValidateUniqueItems::validate_unique_items(
            #field_ident
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            #errors
                .entry(#rename)
                .or_default()
                .push(::serde_valid::validation::Error::UniqueItems(
                    ::serde_valid::error::Message::new(
                        error_params,
                        #message_fn,
                    )
                ));
        }
    )
}
