mod unique_items_from_meta_list;
mod unique_items_from_meta_path;

use proc_macro2::TokenStream;
use quote::quote;
pub use unique_items_from_meta_list::extract_array_unique_items_validator_from_meta_list;
pub use unique_items_from_meta_path::extract_array_unique_items_validator_from_meta_path;

const VALIDATION_LABEL: &'static str = "unique_items";

fn inner_extract_array_unique_items_validator(
    field_name: &str,
    field_ident: &syn::Ident,
    message: TokenStream,
) -> TokenStream {
    quote!(
        if !::serde_valid::validate_array_unique_items(
            #field_ident
        ) {
            use ::serde_valid::validation::error::ToDefaultMessage;
            errors
                .entry(::serde_valid::FieldName::new(#field_name))
                .or_default()
                .push(::serde_valid::validation::Error::UniqueItems(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::UniqueItemsParams::new(
                            #field_ident,
                        ),
                        #message
                    )
                ));
        }
    )
}
