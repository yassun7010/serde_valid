use crate::abort::{abort_duplicated_lit_argument, abort_invalid_attribute_on_field};
use crate::helper::NamedField;
use crate::lit::LitNumeric;
use crate::validator::common::extract_message_tokens;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_numeric_multiples_validator_from_name_value(
    field: &NamedField,
    lit: &syn::Lit,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(
            extract_numeric_multiples_validator_from_name_value(&array_field, lit),
        ))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(
            extract_numeric_multiples_validator_from_name_value(&option_field, lit),
        ))
    } else {
        Validator::Normal(inner_extract_numeric_multiples_validator_from_name_value(
            field.ident(),
            lit,
        ))
    }
}

pub fn extract_numeric_multiple_of_validator_from_list(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_numeric_multiple_of_validator_from_list(
            &array_field,
            attribute,
            meta_items,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_numeric_multiple_of_validator_from_list(
            &option_field,
            attribute,
            meta_items,
        )))
    } else {
        Validator::Normal(inner_extract_numeric_multiple_of_validator_from_list(
            field.ident(),
            attribute,
            meta_items,
        ))
    }
}

fn inner_extract_numeric_multiple_of_validator_from_list(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let multiple_of = get_multiple_of_from_list(field_ident, attribute, meta_items);
    let message = extract_message_tokens("range", field_ident, attribute, meta_items).unwrap_or(
        quote!(::serde_valid::validation::error::MultiplesErrorParams::to_default_message),
    );
    inner_extract_numeric_multiple_of_validator(field_ident, multiple_of, message)
}

fn inner_extract_numeric_multiples_validator_from_name_value(
    field_ident: &syn::Ident,
    lit: &syn::Lit,
) -> TokenStream {
    let multiple_of = get_multiple_of(field_ident, lit);
    let message =
        quote!(::serde_valid::validation::error::MultiplesErrorParams::to_default_message);
    inner_extract_numeric_multiple_of_validator(field_ident, multiple_of, message)
}

fn inner_extract_numeric_multiple_of_validator(
    field_ident: &syn::Ident,
    multiple_of: crate::lit::LitNumeric,
    message: TokenStream,
) -> TokenStream {
    let field_string = field_ident.to_string();
    quote!(
        if !::serde_valid::validate_numeric_multiples(
            *#field_ident,
            #multiple_of,
        ) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_default()
                .push(::serde_valid::validation::Error::MultiplesError(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::MultiplesErrorParams::new(
                            *#field_ident,
                            #multiple_of,
                        ),
                        #message
                    )
                ));
        }
    )
}

fn get_multiple_of_from_list(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> LitNumeric {
    let mut multiple_of = None;
    for meta in meta_items {
        match meta {
            syn::NestedMeta::Lit(lit) => {
                if multiple_of.is_some() {
                    abort_duplicated_lit_argument("multiple_of", field_ident, lit.span());
                }
                multiple_of = Some(get_multiple_of(field_ident, lit));
            }
            syn::NestedMeta::Meta(_) => (),
        }
    }
    multiple_of.unwrap_or_else(|| {
        abort_invalid_attribute_on_field(
            field_ident,
            attribute.span(),
            "Validator `multiple_of` requires at least 1 argument from literal or `message_fn`",
        )
    })
}

fn get_multiple_of(field_ident: &syn::Ident, lit: &syn::Lit) -> LitNumeric {
    match lit {
        syn::Lit::Int(l) => LitNumeric::Int(l.to_owned()),
        syn::Lit::Float(l) => LitNumeric::Float(l.to_owned()),
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
            "invalid argument type for `multiple_of` validator: only numeric literals are allowed",
        ),
    }
}
