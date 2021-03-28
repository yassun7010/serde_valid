use crate::helper::NamedField;
use crate::validator::common::{check_meta, extract_message_tokens};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

const VALIDATION_LABEL: &'static str = "enumerate";

pub fn extract_generic_enumerate_validator(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_generic_enumerate_validator(
            &array_field,
            attribute,
            meta_list,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_generic_enumerate_validator(
            &option_field,
            attribute,
            meta_list,
        )))
    } else {
        Validator::Normal(inner_extract_generic_enumerate_validator(
            field.ident(),
            attribute,
            meta_list,
        ))
    }
}

fn inner_extract_generic_enumerate_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> TokenStream {
    let field_string = field_ident.to_string();
    let syn::MetaList { nested, .. } = meta_list;

    let enumerate = get_enumerate(field_ident, attribute, nested);
    let message = extract_message_tokens(VALIDATION_LABEL, field_ident, attribute, nested)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::EnumerateParams::to_default_message
        ));

    quote!(
        if !::serde_valid::validate_generic_enumerate(
            #field_ident,
            &[#enumerate],
        ) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_default()
                .push(::serde_valid::validation::Error::Enumerate(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::EnumerateParams::new(
                            #field_ident,
                            &[#enumerate],
                        ),
                        #message
                )
                ));
        }
    )
}

fn get_enumerate<'a>(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &'a syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> syn::punctuated::Punctuated<&'a syn::Lit, syn::token::Comma> {
    let mut enumerate = syn::punctuated::Punctuated::<&syn::Lit, syn::token::Comma>::new();
    for meta_item in meta_items {
        match meta_item {
            syn::NestedMeta::Lit(lit) => enumerate.push(lit),
            syn::NestedMeta::Meta(meta) => {
                check_meta(VALIDATION_LABEL, field_ident, meta.span(), meta, true)
            }
        }
    }

    if enumerate.len() == 0 {
        abort!(
            attribute.span(),
            &format!(
                "'{}' literal meta items size must be greater than 0.",
                VALIDATION_LABEL
            )
        )
    }
    enumerate
}
