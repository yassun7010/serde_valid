use crate::helper::NamedField;
use crate::validator::common::{extract_length_validator_tokens, extract_message_tokens};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

const VALIDATION_LABEL: &'static str = "length";
const MIN_LABEL: &'static str = "min_length";
const MAX_LABEL: &'static str = "max_length";

pub fn extract_string_length_validator(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        match array_field.ty() {
            syn::Type::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    if ["u8", "char"].contains(&format!("{}", ident).as_str()) {
                        return Validator::Normal(inner_extract_string_length_validator(
                            field.ident(),
                            attribute,
                            meta_items,
                        ));
                    }
                }
            }
            _ => (),
        }
        Validator::Array(Box::new(extract_string_length_validator(
            &array_field,
            attribute,
            meta_items,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_string_length_validator(
            &option_field,
            attribute,
            meta_items,
        )))
    } else {
        Validator::Normal(inner_extract_string_length_validator(
            field.ident(),
            attribute,
            meta_items,
        ))
    }
}

fn inner_extract_string_length_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let field_string = field_ident.to_string();
    let (min_length_tokens, max_length_tokens) = extract_length_validator_tokens(
        VALIDATION_LABEL,
        MIN_LABEL,
        MAX_LABEL,
        field_ident,
        attribute,
        meta_items,
    );
    let message = extract_message_tokens(VALIDATION_LABEL, field_ident, attribute, meta_items)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::LengthErrorParams::to_default_message
        ));

    quote!(
        if !::serde_valid::validate_string_length(
            #field_ident,
            #min_length_tokens,
            #max_length_tokens
        ) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_default()
                .push(::serde_valid::validation::Error::LengthError(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::LengthErrorParams::new(
                            #field_ident,
                            #min_length_tokens,
                            #max_length_tokens
                        ),
                        #message
                    )
                ));
        }
    )
}
