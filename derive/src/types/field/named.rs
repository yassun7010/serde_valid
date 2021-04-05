use crate::types::{extract_element_type_from_array, extract_type_from_option, Field};
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
pub struct NamedField {
    name: String,
    field: syn::Field,
}

impl NamedField {
    pub fn new(field: syn::Field) -> Self {
        if field.ident.is_none() {
            abort!(field.span(), "struct must be named fields struct.")
        }
        Self {
            name: field.ident.as_ref().unwrap().to_string(),
            field,
        }
    }
}

impl Field for NamedField {
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

    fn array_field(&self) -> Option<NamedField> {
        if let Some(ty) = extract_element_type_from_array(&self.field.ty) {
            Some(NamedField {
                name: self.name.to_owned(),
                field: syn::Field {
                    attrs: self.field.attrs.to_owned(),
                    vis: self.vis().to_owned(),
                    ident: Some(syn::Ident::new(
                        &format!(
                            "_elem_{}",
                            &self.ident().to_string().trim_start_matches("_")
                        ),
                        self.ident().span(),
                    )),
                    colon_token: self.field.colon_token,
                    ty: ty,
                },
            })
        } else {
            None
        }
    }

    fn option_field(&self) -> Option<NamedField> {
        if let Some(ty) = extract_type_from_option(&self.field.ty) {
            Some(NamedField {
                name: self.name.to_owned(),
                field: syn::Field {
                    attrs: self.field.attrs.to_owned(),
                    vis: self.vis().to_owned(),
                    ident: Some(syn::Ident::new(
                        &format!(
                            "_some_{}",
                            &self.ident().to_string().trim_start_matches("_")
                        ),
                        self.ident().span(),
                    )),
                    colon_token: self.field.colon_token,
                    ty: ty,
                },
            })
        } else {
            None
        }
    }
}
