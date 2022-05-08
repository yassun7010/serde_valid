use proc_macro2::TokenStream;
use quote::ToTokens;

pub enum LitNumeric<'a> {
    Int(&'a syn::LitInt),
    Float(&'a syn::LitFloat),
}

impl<'a> ToTokens for LitNumeric<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LitNumeric::Int(lin) => lin.to_tokens(tokens),
            LitNumeric::Float(lin) => lin.to_tokens(tokens),
        }
    }
}

pub fn get_numeric<'a>(lit: &'a syn::Lit) -> Result<LitNumeric<'a>, crate::Errors> {
    match lit {
        syn::Lit::Int(int) => Ok(LitNumeric::Int(int)),
        syn::Lit::Float(float) => Ok(LitNumeric::Float(float)),
        _ => Err(vec![crate::Error::numeric_literal_only(lit)]),
    }
}

pub fn get_str(lit: &syn::Lit) -> Result<&syn::LitStr, crate::Errors> {
    match lit {
        syn::Lit::Str(lit_str) => Ok(lit_str),
        _ => Err(vec![crate::Error::str_literal_only(lit)]),
    }
}
