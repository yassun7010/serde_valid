mod enum_derive;
mod named_struct_derive;
mod unnamed_struct_derive;

use enum_derive::expand_enum_validate_derive;
use named_struct_derive::expand_named_struct_derive;
use proc_macro2::TokenStream;
use unnamed_struct_derive::expand_unnamed_struct_derive;

pub fn expand_derive(input: &syn::DeriveInput) -> Result<TokenStream, crate::Errors> {
    match &input.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => match fields {
            syn::Fields::Named(fields) => expand_named_struct_derive(input, fields),
            syn::Fields::Unnamed(fields) => expand_unnamed_struct_derive(input, fields),
            syn::Fields::Unit => Err(vec![crate::Error::unit_struct_not_supported(input)]),
        },
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            expand_enum_validate_derive(input, variants)
        }
        syn::Data::Union(_) => Err(vec![crate::Error::union_not_supported(input)]),
    }
}
