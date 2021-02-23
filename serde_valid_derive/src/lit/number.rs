use proc_macro2::TokenStream;
use quote::ToTokens;

pub enum LitNumber {
    Int(syn::LitInt),
    Float(syn::LitFloat),
}

pub struct Number {
    lit: LitNumber,
    path_ident: syn::Ident,
}

impl Number {
    pub fn new(lit: LitNumber, path_ident: syn::Ident) -> Self {
        Self { lit, path_ident }
    }
    pub fn path_ident(&self) -> &syn::Ident {
        &self.path_ident
    }

    pub fn path_name(&self) -> String {
        self.path_ident.to_string()
    }
}

impl ToTokens for LitNumber {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LitNumber::Int(lin) => lin.to_tokens(tokens),
            LitNumber::Float(lin) => lin.to_tokens(tokens),
        }
    }
}

impl ToTokens for Number {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lit.to_tokens(tokens)
    }
}
