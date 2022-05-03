mod enum_derive;
mod named_struct_derive;
mod unnamed_struct_derive;

use enum_derive::expand_enum_validate_derive;
use named_struct_derive::expand_named_struct_derive;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::spanned::Spanned;
use unnamed_struct_derive::expand_unnamed_struct_derive;

pub fn expand_derive(input: &syn::DeriveInput) -> Result<TokenStream, Vec<syn::Error>> {
    match &input.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => match fields {
            syn::Fields::Named(named) => Ok(expand_named_struct_derive(input, named)),
            syn::Fields::Unnamed(unnamed) => Ok(expand_unnamed_struct_derive(input, unnamed)),
            syn::Fields::Unit => abort!(
                input.span(),
                "#[derive(Validate)] not support unit field struct"
            ),
        },
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            Ok(expand_enum_validate_derive(input, variants))
        }
        syn::Data::Union(_) => abort!(input.span(), "#[derive(Validate)] not support union data"),
    }
}
