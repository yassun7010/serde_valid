use crate::validator::collect_validators;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, TokenStreamExt};
use syn::spanned::Spanned;

pub fn expand_derive(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let mut validators = TokenStream::new();
    validators.append_all(collect_validators(get_struct_fields(input)));

    let impl_tokens = quote!(
        impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
            fn validate(&self) -> ::std::result::Result<(), Vec<::serde_valid::Error>> {
                let mut errors = vec![];

                #validators

                if errors.is_empty() {
                    ::std::result::Result::Ok(())
                } else {
                    ::std::result::Result::Err(errors)
                }
            }
        }
    );
    impl_tokens
}

fn get_struct_fields(input: &syn::DeriveInput) -> &syn::Fields {
    match input.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => fields,
        _ => abort!(
            input.span(),
            "#[derive(Validate)] can only be used with structs"
        ),
    }
}
