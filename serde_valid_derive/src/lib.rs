use proc_macro::TokenStream;
mod validate;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    validate::expand_derive(&input).into()
}
