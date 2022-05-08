mod array;
mod named;
mod option;
mod unnamed;

pub use named::NamedField;
pub use unnamed::UnnamedField;

pub trait Field {
    fn name(&self) -> &String;

    fn ident(&self) -> &syn::Ident;

    fn getter_token(&self) -> proc_macro2::TokenStream;

    fn attrs(&self) -> &Vec<syn::Attribute>;

    fn vis(&self) -> &syn::Visibility;

    fn ty(&self) -> &syn::Type;

    fn array_field(&self) -> Option<Self>
    where
        Self: Sized;

    fn option_field(&self) -> Option<Self>
    where
        Self: Sized;
}
