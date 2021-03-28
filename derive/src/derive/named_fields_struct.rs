use crate::validator::collect_validators;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

pub fn expand_derive_nameds_fields_struct(
    input: &syn::DeriveInput,
    fields: &syn::FieldsNamed,
) -> TokenStream {
    let ident = &input.ident;
    let validators = TokenStream::from_iter(
        collect_validators(fields)
            .iter()
            .map(|validator| validator.generate_token()),
    );
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let impl_tokens = quote!(
        impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
            fn validate(
                &self
            ) -> ::std::result::Result<(), ::serde_valid::validation::Errors> {
                use ::serde_valid::validation::error::ToDefaultMessage;
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
    );
    impl_tokens
}
