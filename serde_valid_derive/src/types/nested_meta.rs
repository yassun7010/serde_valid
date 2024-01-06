use quote::ToTokens;

pub enum NestedMeta {
    Lit(syn::Lit),
    Meta(syn::Meta),
}

impl ToTokens for NestedMeta {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            NestedMeta::Lit(lit) => lit.to_tokens(tokens),
            NestedMeta::Meta(meta) => meta.to_tokens(tokens),
        }
    }
}

impl syn::parse::Parse for NestedMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Lit) {
            Ok(NestedMeta::Lit(input.parse()?))
        } else if lookahead.peek(syn::Ident) {
            Ok(NestedMeta::Meta(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}
