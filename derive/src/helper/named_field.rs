use super::extract_type_from_array;
use super::extract_type_from_option;
use crate::abort::abort_unnamed_fields_struct;
use ref_cast::RefCast;
use std::convert::AsRef;
use syn::spanned::Spanned;

#[derive(RefCast)]
#[repr(transparent)]
pub struct NamedField(syn::Field);

impl<'a> NamedField {
    pub fn new(field: syn::Field) -> Self {
        if field.ident.is_none() {
            abort_unnamed_fields_struct(field.span())
        }
        Self(field)
    }

    #[allow(dead_code)]
    pub fn ident(&self) -> &syn::Ident {
        self.0.ident.as_ref().unwrap()
    }

    #[allow(dead_code)]
    pub fn attrs(&self) -> &Vec<syn::Attribute> {
        self.0.attrs.as_ref()
    }

    #[allow(dead_code)]
    pub fn vis(&self) -> &syn::Visibility {
        &self.0.vis
    }

    #[allow(dead_code)]
    pub fn ty(&self) -> &syn::Type {
        &self.0.ty
    }

    #[allow(dead_code)]
    pub fn array_field(&self) -> Option<NamedField> {
        if let Some(ty) = extract_type_from_array(&self.0.ty) {
            Some(NamedField::new(syn::Field {
                attrs: vec![],
                vis: self.vis().to_owned(),
                ident: Some(syn::Ident::new(
                    &format!("_{}", &self.ident()),
                    self.ident().span(),
                )),
                colon_token: None,
                ty: ty,
            }))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn option_field(&self) -> Option<NamedField> {
        if let Some(ty) = extract_type_from_option(&self.0.ty) {
            Some(NamedField::new(syn::Field {
                attrs: vec![],
                vis: self.vis().to_owned(),
                ident: Some(syn::Ident::new(
                    &format!("_{}", &self.ident()),
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
