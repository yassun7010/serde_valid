mod multiples;
mod range;
use super::abort::{abort_invalid_attribute_on_field, abort_unnamed_fields_struct};
use multiples::extract_multiples_validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens, TokenStreamExt};
use range::extract_range_validator;
use syn::{parse_quote, spanned::Spanned};

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

/// Find the types (as string) for each field of the struct
/// Needed for the `must_match` filter
fn collect_validators(fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    let mut validators = vec![];
    for field in fields {
        let field_ident = &field
            .ident
            .as_ref()
            .unwrap_or_else(|| abort_unnamed_fields_struct(field.span()));
        for attribute in &field.attrs {
            if attribute.path != parse_quote!(validate) {
                continue;
            }
            let validator = extract_validator(field_ident, attribute);
            match validator {
                Some(validator) => validators.push(validator),
                None => abort_invalid_attribute_on_field(
                    &field_ident,
                    attribute.span(),
                    "it needs at least one validator",
                ),
            }
        }
    }

    validators
}

fn extract_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
) -> Option<proc_macro2::TokenStream> {
    match attribute.parse_meta() {
        Ok(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
            // only validation from there on
            for meta_item in nested {
                match meta_item {
                    syn::NestedMeta::Meta(item) => match item {
                        // Validators with several args
                        syn::Meta::List(syn::MetaList { path, nested, .. }) => {
                            let ident = path.get_ident().unwrap();
                            match ident.to_string().as_ref() {
                                "range" => {
                                    return Some(extract_range_validator(
                                        &field_ident,
                                        &attribute,
                                        &nested,
                                    ))
                                }
                                v => {
                                    abort!(path.span(), "unexpected list validator: {:?}", v)
                                }
                            }
                        }
                        syn::Meta::NameValue(syn::MetaNameValue { path, lit, .. }) => {
                            let ident = path.get_ident().unwrap();
                            match ident.to_string().as_ref() {
                                "multiple_of" => {
                                    return Some(extract_multiples_validator(field_ident, lit))
                                }
                                v => {
                                    abort!(path.span(), "unexpected name value validator: {:?}", v)
                                }
                            }
                        }
                        _ => abort!(item.span(), "unsupport non MetaList Meta type"),
                    },
                    _ => unreachable!("Found a non Meta while looking for validators"),
                };
            }
        }
        // TODO
        Ok(syn::Meta::Path(_)) => abort!(attribute.span(), "Support nested arguments"),
        Ok(syn::Meta::NameValue(_)) => {
            abort!(attribute.span(), "Unexpected name=value argument")
        }
        Err(e) => unreachable!(
            "Got something other than a list of attributes while checking field `{}`: {:?}",
            field_ident, e
        ),
    }
    None
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
