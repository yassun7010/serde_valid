use crate::abort::{
    abort_duplicated_argument, abort_invalid_attribute_on_field, abort_unexpected_list_argument,
    abort_unexpected_path_argument,
};
use crate::helper::{NamedField, SingleIdentPath};
use crate::lit::{LitNumeric, NumericInfo};
use crate::validator::common::extract_message_tokens;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_numeric_range_validator(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_numeric_range_validator(
            &array_field,
            attribute,
            meta_items,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_numeric_range_validator(
            &option_field,
            attribute,
            meta_items,
        )))
    } else {
        Validator::Normal(inner_extract_numeric_range_validator(
            field.ident(),
            attribute,
            meta_items,
        ))
    }
}

fn inner_extract_numeric_range_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let field_string = field_ident.to_string();
    let (minimum_tokens, maximum_tokens) =
        extract_numeric_range_validator_tokens(field_ident, attribute, meta_items);
    let message = extract_message_tokens("range", field_ident, attribute, meta_items).unwrap_or(
        quote!(::serde_valid::validation::error::RangeErrorParams::to_default_message),
    );

    quote!(
        if !::serde_valid::validate_numeric_range(
            *#field_ident,
            #minimum_tokens,
            #maximum_tokens
        ) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_default()
                .push(::serde_valid::validation::Error::RangeError(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::RangeErrorParams::new(
                            *#field_ident,
                            #minimum_tokens,
                            #maximum_tokens
                        ),
                        #message
                    )
                ));
        }
    )
}

fn extract_numeric_range_validator_tokens(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> (TokenStream, TokenStream) {
    let mut minimum = None;
    let mut exclusive_minimum = None;
    let mut maximum = None;
    let mut exclusive_maximum = None;
    for meta_item in meta_items {
        if let syn::NestedMeta::Meta(ref item) = *meta_item {
            match item {
                syn::Meta::NameValue(name_value) => update_limit(
                    field_ident,
                    name_value,
                    &mut minimum,
                    &mut exclusive_minimum,
                    &mut maximum,
                    &mut exclusive_maximum,
                ),
                syn::Meta::List(list) => {
                    abort_unexpected_list_argument("range", field_ident, item.span(), list, true);
                }
                syn::Meta::Path(path) => {
                    abort_unexpected_path_argument("range", field_ident, item.span(), path)
                }
            }
        }
    }
    let minimum_tokens = get_limit_tokens(field_ident, minimum, exclusive_minimum);
    let maximum_tokens = get_limit_tokens(field_ident, maximum, exclusive_maximum);

    if minimum_tokens.to_string() == "None" && maximum_tokens.to_string() == "None" {
        abort_invalid_attribute_on_field(
            field_ident,
            attribute.span(),
            "Validator `range` requires at least 1 argument from `minimum` or `exclusive_minimum`, `maximum` or `exclusive_maximum`",
        );
    }
    (minimum_tokens, maximum_tokens)
}

fn update_limit(
    field_ident: &syn::Ident,
    syn::MetaNameValue { path, lit, .. }: &syn::MetaNameValue,
    minimum: &mut Option<NumericInfo>,
    exclusive_minimum: &mut Option<NumericInfo>,
    maximum: &mut Option<NumericInfo>,
    exclusive_maximum: &mut Option<NumericInfo>,
) {
    let path_ident = SingleIdentPath::new(path).ident();
    match path_ident.to_string().as_ref() {
        "minimum" => {
            update_numeric(minimum, field_ident, lit, path_ident);
        }
        "exclusive_minimum" => update_numeric(exclusive_minimum, field_ident, lit, path_ident),
        "maximum" => update_numeric(maximum, field_ident, lit, path_ident),
        "exclusive_maximum" => update_numeric(exclusive_maximum, field_ident, lit, path_ident),
        v => abort_invalid_attribute_on_field(
            field_ident,
            path.span(),
            &format!(
                "Unknown argument `{}` for validator `range` \
            (it only has [`minimum` or `exclusive_minimum`, \
            `maximum` or `exclusive_maximum`, `message_fn`])",
                v
            ),
        ),
    }
}

fn update_numeric(
    target: &mut Option<NumericInfo>,
    field_ident: &syn::Ident,
    lit: &syn::Lit,
    path_ident: &syn::Ident,
) {
    if target.is_some() {
        abort_duplicated_argument("range", field_ident, lit.span(), path_ident)
    }

    match lit {
        syn::Lit::Int(l) => {
            *target = Some(NumericInfo::new(LitNumeric::Int(l.to_owned()), path_ident.to_owned()))
        },
        syn::Lit::Float(l) => {
            *target = Some(NumericInfo::new(LitNumeric::Float(l.to_owned()), path_ident.to_owned()))
        },
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
             &format!("Invalid argument type for `{}` of `range` validator: only numeric literals are allowed", path_ident.to_string())),
    }
}

fn get_limit_tokens(
    field_ident: &syn::Ident,
    inclusive_limit: Option<NumericInfo>,
    exclusive_limit: Option<NumericInfo>,
) -> proc_macro2::TokenStream {
    match (inclusive_limit, exclusive_limit) {
        (Some(inclusive), Some(exclusive)) => abort_invalid_attribute_on_field(
            field_ident,
            inclusive
                .path_ident()
                .span()
                .join(exclusive.path_ident().span())
                .unwrap_or(inclusive.path_ident().span()),
            &format!(
                "Both `{}` and `{}` have been set in `range` validator: conflict",
                inclusive.path_name(),
                exclusive.path_name()
            ),
        ),
        (Some(inclusive_limit), None) => {
            quote!(Some(::serde_valid::Limit::Inclusive(#inclusive_limit)))
        }
        (None, Some(exclusive_limit)) => {
            quote!(Some(::serde_valid::Limit::Exclusive(#exclusive_limit)))
        }
        (None, None) => quote!(None),
    }
}
