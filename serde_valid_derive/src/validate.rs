use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

pub fn expand_derive(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let _struct_fields = get_struct_fields(input);
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    let impl_tokens = quote!(
        impl #impl_generics ::serde_valid::Validate for #ident #type_generics #where_clause {
            fn validate(&self) -> ::std::result::Result<(), Vec<::serde_valid::Error>> {
                let mut errors = vec![];

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

fn get_struct_fields(input: &syn::DeriveInput) -> Vec<&syn::Field> {
    match input.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => fields
            .iter()
            .map(|field| {
                if field.ident.is_none() {
                    abort!(
                        fields.span(),
                        "struct has unnamed fields";
                        help = "#[derive(Validate)] can only be used on structs with named fields";
                    );
                };
                field
            })
            .collect::<Vec<_>>(),
        _ => abort!(
            input.span(),
            "#[derive(Validate)] can only be used with structs"
        ),
    }
}
