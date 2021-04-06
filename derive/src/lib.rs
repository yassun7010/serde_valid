use proc_macro::TokenStream;
mod abort;
mod derive;
mod errors;
mod lit;
mod types;
mod validator;

use derive::expand_derive;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Validate, attributes(validate))]
#[proc_macro_error]
pub fn derive_validate(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    expand_derive(&input).into()
}
