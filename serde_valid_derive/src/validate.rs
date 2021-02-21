use quote::quote;

pub fn expand_derive(_ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let impl_ast = quote! {};
    impl_ast
}
