use proc_macro2::TokenStream;
use quote::ToTokens;

pub enum LitNumeric {
    Int(syn::LitInt),
    Float(syn::LitFloat),
}

pub struct NumericInfo {
    lit: LitNumeric,
    path_ident: syn::Ident,
}

impl NumericInfo {
    pub fn new(lit: LitNumeric, path_ident: syn::Ident) -> Self {
        Self { lit, path_ident }
    }
    pub fn path_ident(&self) -> &syn::Ident {
        &self.path_ident
    }

    pub fn path_name(&self) -> String {
        self.path_ident.to_string()
    }
}

impl ToTokens for LitNumeric {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LitNumeric::Int(lin) => lin.to_tokens(tokens),
            LitNumeric::Float(lin) => lin.to_tokens(tokens),
        }
    }
}

impl ToTokens for NumericInfo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lit.to_tokens(tokens)
    }
}
