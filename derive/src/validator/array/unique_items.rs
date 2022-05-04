mod unique_items_from_meta_list;
mod unique_items_from_meta_path;

use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;
pub use unique_items_from_meta_list::extract_array_unique_items_validator_from_meta_list;
pub use unique_items_from_meta_path::extract_array_unique_items_validator_from_meta_path;

const VALIDATION_LABEL: &'static str = "unique_items";

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
