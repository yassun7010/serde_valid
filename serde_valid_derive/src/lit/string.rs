use proc_macro2::TokenStream;
use quote::ToTokens;

#[allow(dead_code)]
pub enum LitString {
    Str(syn::LitStr),
    ByteStr(syn::LitByteStr),
}

impl ToTokens for LitString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LitString::Str(lin) => lin.to_tokens(tokens),
            LitString::ByteStr(lin) => lin.to_tokens(tokens),
        }
    }
}
