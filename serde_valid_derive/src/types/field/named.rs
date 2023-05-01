use super::Field;
use proc_macro_error::abort;
use quote::quote;
use std::borrow::Cow;
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
pub struct NamedField<'a> {
    name: String,
    field: Cow<'a, syn::Field>,
}

impl<'a> NamedField<'a> {
    pub fn new(field: &'a syn::Field) -> Self {
        if field.ident.is_none() {
            abort!(field.span(), "struct must be named fields struct.")
        }
        Self {
            name: field.ident.as_ref().unwrap().to_string(),
            field: Cow::Borrowed(field),
        }
    }
}

impl<'a> Field for NamedField<'a> {
    fn name(&self) -> &String {
        &self.name
    }

    fn ident(&self) -> &syn::Ident {
        self.field.ident.as_ref().unwrap()
    }

    fn key(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        quote!(#name.to_string())
    }

    fn errors_variable(&self) -> proc_macro2::TokenStream {
        quote!(__property_vec_errors_map)
    }

    fn getter_token(&self) -> proc_macro2::TokenStream {
        let ident = self.ident();
        quote!(#ident)
    }

    fn attrs(&self) -> &Vec<syn::Attribute> {
        self.field.attrs.as_ref()
    }

    fn vis(&self) -> &syn::Visibility {
        &self.field.vis
    }

    fn ty(&self) -> &syn::Type {
        &self.field.ty
    }
}
