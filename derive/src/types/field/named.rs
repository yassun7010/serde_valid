use super::array::{extract_element_type_from_array, make_element_field};
use super::option::{extract_type_from_option, make_some_field};
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

    fn ident_tokens(&self) -> proc_macro2::TokenStream {
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

    fn array_field(&self) -> Option<NamedField<'a>> {
        if let Some(ty) = extract_element_type_from_array(&self.field.ty) {
            Some(NamedField {
                name: self.name.clone(),
                field: Cow::Owned(make_element_field(&self.field, self.field.span(), ty)),
            })
        } else {
            None
        }
    }

    fn option_field(&self) -> Option<NamedField<'a>> {
        if let Some(ty) = extract_type_from_option(&self.field.ty) {
            Some(NamedField {
                name: self.name.clone(),
                field: Cow::Owned(make_some_field(&self.field, self.field.span(), ty)),
            })
        } else {
            None
        }
    }
}
