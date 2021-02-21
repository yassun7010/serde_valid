use proc_macro_error::proc_macro_error;
use quote::quote;

#[proc_macro_derive(Validate, attributes(validate))]
#[proc_macro_error]
pub fn derive_validation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_validate(&ast).into()
}

fn impl_validate(_ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let impl_ast = quote! {};
    impl_ast
}
