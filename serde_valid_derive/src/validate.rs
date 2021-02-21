use quote::quote;

pub fn expand_derive(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
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
