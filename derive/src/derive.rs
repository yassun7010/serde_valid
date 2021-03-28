mod named_fields_struct;

use named_fields_struct::expand_derive_nameds_fields_struct;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn expand_derive(input: &syn::DeriveInput) -> TokenStream {
    match input.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => match fields {
            syn::Fields::Named(named) => expand_derive_nameds_fields_struct(input, named),
            syn::Fields::Unnamed(_) => abort!(
                input.span(),
                "#[derive(Validate)] can only be used with named field structs"
            ),
            syn::Fields::Unit => abort!(
                input.span(),
                "#[derive(Validate)] cannot be used with unit field structs"
            ),
        },
        _ => abort!(
            input.span(),
            "#[derive(Validate)] can only be used with named field structs"
        ),
    }
}
