use crate::types::Field;
use crate::validator::common::{check_validation_arg_meta, extract_message_tokens};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

const VALIDATION_LABEL: &'static str = "enumerate";

pub fn extract_generic_enumerate_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_generic_enumerate_validator(
            &array_field,
            attribute,
            validation_list,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_generic_enumerate_validator(
            &option_field,
            attribute,
            validation_list,
        )))
    } else {
        Validator::Normal(inner_extract_generic_enumerate_validator(
            field,
            attribute,
            validation_list,
        ))
    }
}

fn inner_extract_generic_enumerate_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_list: &syn::MetaList,
) -> TokenStream {
    let syn::MetaList {
        nested: validation_args,
        ..
    } = validation_list;
    let field_name = field.name();
    let field_ident = field.ident();

    let enumerate = get_enumerate(field, attribute, validation_args);
    let message = extract_message_tokens(VALIDATION_LABEL, field, attribute, validation_args)
        .unwrap_or(quote!(::serde_valid::EnumerateParams::to_default_message));

    quote!(
        if !::serde_valid::validate_generic_enumerate(
            #field_ident,
            &[#enumerate],
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            __errors
                .entry(#field_name)
                .or_default()
                .push(::serde_valid::validation::Error::Enumerate(
                    ::serde_valid::error::Message::new(
                        ::serde_valid::EnumerateParams::new(
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
    field: &impl Field,
    attribute: &syn::Attribute,
    validation_args: &'a syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> syn::punctuated::Punctuated<&'a syn::Lit, syn::token::Comma> {
    let mut enumerate = syn::punctuated::Punctuated::<&syn::Lit, syn::token::Comma>::new();
    for validation_arg in validation_args {
        match validation_arg {
            syn::NestedMeta::Lit(lit) => enumerate.push(lit),
            syn::NestedMeta::Meta(arg_meta) => {
                check_validation_arg_meta(VALIDATION_LABEL, field, arg_meta, true)
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
