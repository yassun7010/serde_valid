mod named;
mod unnamed;

pub use named::NamedField;
pub use unnamed::UnnamedField;

pub trait Field {
    fn name(&self) -> &String;

    fn ident(&self) -> &syn::Ident;

    fn key(&self) -> proc_macro2::TokenStream;

    fn errors_variable(&self) -> proc_macro2::TokenStream;

    fn getter_token(&self) -> proc_macro2::TokenStream;

    fn attrs(&self) -> &Vec<syn::Attribute>;

    #[allow(dead_code)]
    fn vis(&self) -> &syn::Visibility;

    #[allow(dead_code)]
    fn ty(&self) -> &syn::Type;
}
