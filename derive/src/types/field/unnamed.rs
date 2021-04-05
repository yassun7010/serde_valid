use crate::types::array::{make_element_field, make_element_ident};
use crate::types::option::{make_some_field, make_some_ident};
use crate::types::{extract_element_type_from_array, extract_type_from_option, Field};
use proc_macro_error::abort;
use quote::quote;
use std::convert::AsRef;
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
pub struct UnnamedField {
    name: String,
    index: usize,
    ident: syn::Ident,
    field: syn::Field,
}

impl UnnamedField {
    pub fn new(index: usize, field: syn::Field) -> Self {
        if field.ident.is_some() {
            abort!(field.span(), "struct must be unnamed fields struct.")
        }
        Self {
            name: index.to_string(),
            index,
            ident: syn::Ident::new(&format!("_{}", index), field.span()),
            field,
        }
    }
}

impl Field for UnnamedField {
    fn name(&self) -> &String {
        &self.name
    }

    fn ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn ident_tokens(&self) -> proc_macro2::TokenStream {
        let index = syn::Index::from(self.index);
        quote!(#index)
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

    fn array_field(&self) -> Option<UnnamedField> {
        if let Some(ty) = extract_element_type_from_array(&self.ty()) {
            Some(UnnamedField {
                index: self.index,
                name: self.name.to_owned(),
                ident: make_element_ident(&self.ident, self.field.span()),
                field: make_element_field(&self.field, self.field.span(), ty),
            })
        } else {
            None
        }
    }

    fn option_field(&self) -> Option<UnnamedField> {
        if let Some(ty) = extract_type_from_option(&self.ty()) {
            Some(UnnamedField {
                index: self.index,
                name: self.name.to_owned(),
                ident: make_some_ident(&self.ident, self.field.span()),
                field: make_some_field(&self.field, self.field.span(), ty),
            })
        } else {
            None
        }
    }
}
