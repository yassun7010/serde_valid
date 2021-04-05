mod enum_variants;
mod struct_named_fields;
mod struct_unnamed_fields;

use enum_variants::expand_enum_variants_validators;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use struct_named_fields::expand_struct_named_fields_validators_tokens;
use struct_unnamed_fields::expand_struct_unnamed_fields_validators_tokens;
use syn::spanned::Spanned;

pub fn expand_derive(input: &syn::DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let validators = match &input.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => match fields {
            syn::Fields::Named(named) => expand_struct_named_fields_validators_tokens(named),
            syn::Fields::Unnamed(unnamed) => {
                expand_struct_unnamed_fields_validators_tokens(unnamed)
            }
            syn::Fields::Unit => abort!(
                input.span(),
                "#[derive(Validate)] not support unit field struct"
            ),
        },
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            expand_enum_variants_validators(ident, variants)
        }
        syn::Data::Union(_) => abort!(input.span(), "#[derive(Validate)] not support union data"),
    };

    quote!(
        impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
            fn validate(
                &self
            ) -> ::std::result::Result<(), ::serde_valid::validation::Errors> {
                let mut errors = ::serde_valid::validation::InnerErrors::new();

                #validators

                if errors.is_empty() {
                    ::std::result::Result::Ok(())
                } else {
                    ::std::result::Result::Err(
                        ::serde_valid::validation::Errors::new(errors)
                    )
                }
            }
        }
    )
}
