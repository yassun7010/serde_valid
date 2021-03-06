mod meta;
mod number;
use crate::abort::abort_invalid_attribute_on_field;
use meta::collect_validators;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens, TokenStreamExt};
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

#[allow(dead_code)]
fn get_field_type(field_type: &syn::Type, field_ident: &syn::Ident) -> String {
    match field_type {
        syn::Type::Path(syn::TypePath { ref path, .. }) => path.to_token_stream().to_string(),
        syn::Type::Reference(syn::TypeReference {
            ref lifetime,
            ref elem,
            ..
        }) => {
            if lifetime.is_some() {
                format!("&{}", elem.to_token_stream())
            } else {
                elem.to_token_stream().to_string()
            }
        }
        _ => {
            abort!(
                field_type.span(),
                "Type `{}` of field `{}` not supported",
                field_type.to_token_stream(),
                field_ident
            )
        }
    }
}

#[allow(dead_code)]
fn find_original_field_name<'a>(meta_items: &[&'a syn::NestedMeta]) -> Option<&'a syn::LitStr> {
    for meta_item in meta_items {
        match **meta_item {
            syn::NestedMeta::Meta(ref item) => match *item {
                syn::Meta::Path(_) => continue,
                syn::Meta::NameValue(syn::MetaNameValue {
                    ref path, ref lit, ..
                }) => {
                    let ident = path.get_ident().unwrap();
                    if ident == "rename" {
                        if let syn::Lit::Str(lit_str) = lit {
                            return Some(lit_str);
                        }
                    }
                }
                syn::Meta::List(syn::MetaList { ref nested, .. }) => {
                    return find_original_field_name(&nested.iter().collect::<Vec<_>>());
                }
            },
            _ => unreachable!(),
        };
    }
    None
}
