mod multiple_of_from_meta_list;
mod multiple_of_from_meta_name_value;

use crate::types::Field;
pub use multiple_of_from_meta_list::extract_numeric_multiple_of_validator_from_meta_list;
pub use multiple_of_from_meta_name_value::extract_numeric_multiple_of_validator_from_meta_name_value;
use proc_macro2::TokenStream;
use quote::quote;

const VALIDATION_LABEL: &'static str = "multiple_of";

fn inner_extract_numeric_multiple_of_validator<F: Field>(
    field: &F,
    multiple_of: crate::lit::LitNumeric,
    message: TokenStream,
) -> TokenStream {
    let field_name = field.name();
    let field_ident = field.ident();

    quote!(
        if !::serde_valid::validate_numeric_multiple_of(
            *#field_ident,
            #multiple_of,
        ) {
            use ::serde_valid::validation::error::ToDefaultMessage;
            __errors
                .entry(#field_name)
                .or_default()
                .push(::serde_valid::validation::Error::MultipleOf(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::MultipleOfParams::new(
                            *#field_ident,
                            #multiple_of,
                        ),
                        #message
                    )
                ));
        }
    )
}
