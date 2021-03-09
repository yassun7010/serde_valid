use crate::abort::abort_unnamed_fields_struct;
use syn::spanned::Spanned;

pub struct NamedField<'a>(&'a syn::Field);

impl<'a> NamedField<'a> {
    pub fn new(field: &'a syn::Field) -> Self {
        if field.ident.is_none() {
            abort_unnamed_fields_struct(field.span())
        }
        Self(field)
    }

    pub fn ident(&self) -> &syn::Ident {
        self.0.ident.as_ref().unwrap()
    }

    pub fn attributes(&self) -> &Vec<syn::Attribute> {
        self.0.attrs.as_ref()
    }
}

pub struct NamedFieldBuf(syn::Field);

impl<'a> NamedFieldBuf {
    pub fn new(field: syn::Field) -> Self {
        if field.ident.is_none() {
            abort_unnamed_fields_struct(field.span())
        }
        Self(field)
    }

    pub fn ident(&self) -> &syn::Ident {
        self.0.ident.as_ref().unwrap()
    }

    #[allow(dead_code)]
    pub fn attributes(&self) -> &Vec<syn::Attribute> {
        self.0.attrs.as_ref()
    }
}
