use crate::types::{extract_element_type_from_array, extract_type_from_option, Field};
use proc_macro_error::abort;
use quote::quote;
use ref_cast::RefCast;
use std::convert::AsRef;
use syn::spanned::Spanned;

#[derive(RefCast)]
#[repr(transparent)]
pub struct NamedField(syn::Field);

impl NamedField {
    pub fn new(field: syn::Field) -> Self {
        if field.ident.is_none() {
            abort!(field.span(), "struct must be named fields struct.")
        }
        Self(field)
    }
}

impl Field for NamedField {
    fn ident(&self) -> &syn::Ident {
        self.0.ident.as_ref().unwrap()
    }

    fn ident_tokens(&self) -> proc_macro2::TokenStream {
        let ident = self.ident();
        quote!(#ident)
    }

    fn attrs(&self) -> &Vec<syn::Attribute> {
        self.0.attrs.as_ref()
    }

    fn vis(&self) -> &syn::Visibility {
        &self.0.vis
    }

    fn ty(&self) -> &syn::Type {
        &self.0.ty
    }

    fn array_field(&self) -> Option<NamedField> {
        if let Some(ty) = extract_element_type_from_array(&self.0.ty) {
            Some(NamedField::new(syn::Field {
                attrs: vec![],
                vis: self.vis().to_owned(),
                ident: Some(syn::Ident::new(
                    &format!(
                        "_elem_{}",
                        &self.ident().to_string().trim_start_matches("_")
                    ),
                    self.ident().span(),
                )),
                colon_token: None,
                ty: ty,
            }))
        } else {
            None
        }
    }

    fn option_field(&self) -> Option<NamedField> {
        if let Some(ty) = extract_type_from_option(&self.0.ty) {
            Some(NamedField::new(syn::Field {
                attrs: vec![],
                vis: self.vis().to_owned(),
                ident: Some(syn::Ident::new(
                    &format!(
                        "_some_{}",
                        &self.ident().to_string().trim_start_matches("_")
                    ),
                    self.ident().span(),
                )),
                colon_token: None,
                ty: ty,
            }))
        } else {
            None
        }
    }
}
