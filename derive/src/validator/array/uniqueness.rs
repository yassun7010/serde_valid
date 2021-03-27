use crate::abort::abort_required_list_argument;
use crate::helper::NamedField;
use crate::validator::common::extract_message_tokens;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

const VALIDATION_LABEL: &'static str = "unique_items";

pub fn extract_array_length_validator_from_list(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_array_length_validator_from_list(
            &option_field,
            attribute,
            meta_list,
        )))
    } else {
        Validator::Normal(inner_extract_array_length_validator_from_list(
            field.ident(),
            attribute,
            meta_list,
        ))
    }
}

pub fn extract_array_uniqueness_validator_from_path(field: &NamedField) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_array_uniqueness_validator_from_path(
            &option_field,
        )))
    } else {
        let message =
            quote!(::serde_valid::validation::error::UniqueItemsErrorParams::to_default_message);
        Validator::Normal(inner_extract_array_uniqueness_validator(
            field.ident(),
            message,
        ))
    }
}

fn inner_extract_array_length_validator_from_list(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> TokenStream {
    let syn::MetaList { nested, .. } = meta_list;

    let message = extract_message_tokens(VALIDATION_LABEL, field_ident, attribute, nested)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::ItemsErrorParams::to_default_message
        ));
    if nested.is_empty() {
        abort_required_list_argument(
            VALIDATION_LABEL,
            &["message_fn"],
            field_ident,
            attribute.span(),
            meta_list,
            true,
        )
    }
    inner_extract_array_uniqueness_validator(field_ident, message)
}

fn inner_extract_array_uniqueness_validator(
    field_ident: &syn::Ident,
    message: TokenStream,
) -> TokenStream {
    let field_string = field_ident.to_string();
    quote!(
        if !::serde_valid::validate_array_uniqueness(
            #field_ident
        ) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_default()
                .push(::serde_valid::validation::Error::UniqueItemsError(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::UniqueItemsErrorParams::new(
                            #field_ident,
                        ),
                        #message
                    )
                ));
        }
    )
}
