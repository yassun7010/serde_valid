use super::named_fields_struct::collect_named_fields_struct_validators;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

type Variants = syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>;

pub fn expand_enum_variants_validators(
    enum_ident: &syn::Ident,
    variants: &Variants,
) -> TokenStream {
    let mut enum_validator_tokens = vec![];
    for variant in variants.iter() {
        let variant_tokens = match &variant.fields {
            syn::Fields::Named(fields_named) => {
                let variant_ident = &variant.ident;
                let fields_validators = collect_named_fields_struct_validators(fields_named);
                let mut fields_idents =
                    syn::punctuated::Punctuated::<TokenStream, syn::Token!(,)>::new();
                let fields_validators_tokens =
                    TokenStream::from_iter(fields_validators.iter().map(|validators| {
                        let field_ident = validators.ident();
                        if let Some(token) = validators.get_tokens() {
                            fields_idents.push(quote!(#field_ident));
                            quote!(#token)
                        } else {
                            fields_idents.push(quote!(#field_ident: _));
                            quote!()
                        }
                    }));
                quote!(
                    if let #enum_ident::#variant_ident{#fields_idents} = &self {
                        #fields_validators_tokens
                    }
                )
            }
            _ => quote!(),
        };
        enum_validator_tokens.push(variant_tokens);
        // abort!(variant.span(), "Variant: {:?}", variant);
    }
    TokenStream::from_iter(enum_validator_tokens)
}
