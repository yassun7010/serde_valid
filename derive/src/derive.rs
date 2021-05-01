mod enum_variants;
mod struct_named_fields;
mod struct_unnamed_fields;

use enum_variants::expand_enum_variants_validate;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use struct_named_fields::expand_struct_named_fields_validate;
use struct_unnamed_fields::expand_struct_unnamed_fields_validate;
use syn::spanned::Spanned;

pub fn expand_derive(input: &syn::DeriveInput) -> TokenStream {
    match &input.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => match fields {
            syn::Fields::Named(named) => expand_struct_named_fields_validate(input, named),
            syn::Fields::Unnamed(unnamed) => expand_struct_unnamed_fields_validate(input, unnamed),
            syn::Fields::Unit => abort!(
                input.span(),
                "#[derive(Validate)] not support unit field struct"
            ),
        },
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            expand_enum_variants_validate(input, variants)
        }
        syn::Data::Union(_) => abort!(input.span(), "#[derive(Validate)] not support union data"),
    }
}
