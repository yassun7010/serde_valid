use proc_macro2::TokenStream;
use quote::ToTokens;

pub enum LitNumeric<'a> {
    Int(&'a syn::LitInt),
    Float(&'a syn::LitFloat),
}

pub struct NumericInfo<'a> {
    lit: LitNumeric<'a>,
    path_ident: syn::Ident,
}

impl<'a> NumericInfo<'a> {
    pub fn new(lit: LitNumeric<'a>, path_ident: syn::Ident) -> Self {
        Self { lit, path_ident }
    }
    pub fn path_ident(&self) -> &syn::Ident {
        &self.path_ident
    }

    pub fn path_name(&self) -> String {
        self.path_ident.to_string()
    }
}

impl<'a> ToTokens for LitNumeric<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LitNumeric::Int(lin) => lin.to_tokens(tokens),
            LitNumeric::Float(lin) => lin.to_tokens(tokens),
        }
    }
}

impl<'a> ToTokens for NumericInfo<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lit.to_tokens(tokens)
    }
}
