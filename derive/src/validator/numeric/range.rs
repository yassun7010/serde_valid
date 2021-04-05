use crate::abort::{
    abort_duplicated_argument, abort_invalid_attribute_on_field, abort_required_path_argument,
    abort_unexpected_list_argument, abort_unexpected_path_argument,
    abort_unknown_name_value_argument,
};
use crate::lit::NumericInfo;
use crate::types::{Field, SingleIdentPath};
use crate::validator::common::{
    check_common_list_argument, check_lit, extract_message_tokens, get_numeric,
};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

const VALIDATION_LABEL: &'static str = "range";
const EXPECTED_KEYS: [&str; 4] = [
    "minimum",
    "exclusive_minimum",
    "maximum",
    "exclusive_maximum",
];

pub fn extract_numeric_range_validator<F: Field>(
    field: &F,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Validator {
    let syn::MetaList { nested, .. } = meta_list;
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_numeric_range_validator(
            &array_field,
            attribute,
            meta_list,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_numeric_range_validator(
            &option_field,
            attribute,
            meta_list,
        )))
    } else {
        Validator::Normal(inner_extract_numeric_range_validator(
            field.name(),
            field.ident(),
            attribute,
            nested,
        ))
    }
}

fn inner_extract_numeric_range_validator(
    field_name: &str,
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let (minimum_tokens, maximum_tokens) =
        extract_numeric_range_validator_tokens(field_ident, attribute, meta_items);
    let message = extract_message_tokens(VALIDATION_LABEL, field_ident, attribute, meta_items)
        .unwrap_or(quote!(
            ::serde_valid::validation::error::RangeParams::to_default_message
        ));

    quote!(
        if !::serde_valid::validate_numeric_range(
            *#field_ident,
            #minimum_tokens,
            #maximum_tokens
        ) {
            use ::serde_valid::validation::error::ToDefaultMessage;
            errors
                .entry(::serde_valid::FieldName::new(#field_name))
                .or_default()
                .push(::serde_valid::validation::Error::Range(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::RangeParams::new(
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
        match meta_item {
            syn::NestedMeta::Meta(ref item) => match item {
                syn::Meta::NameValue(name_value) => update_limit(
                    field_ident,
                    name_value,
                    &mut minimum,
                    &mut exclusive_minimum,
                    &mut maximum,
                    &mut exclusive_maximum,
                ),
                syn::Meta::List(list) => {
                    if !check_common_list_argument(list) {
                        abort_unexpected_list_argument(
                            VALIDATION_LABEL,
                            field_ident,
                            item.span(),
                            list,
                        );
                    };
                }
                syn::Meta::Path(path) => {
                    abort_unexpected_path_argument(VALIDATION_LABEL, field_ident, item.span(), path)
                }
            },
            syn::NestedMeta::Lit(lit) => check_lit(VALIDATION_LABEL, field_ident, lit.span(), lit),
        }
    }
    let minimum_tokens = get_limit_tokens(field_ident, minimum, exclusive_minimum);
    let maximum_tokens = get_limit_tokens(field_ident, maximum, exclusive_maximum);

    if minimum_tokens.to_string() == "None" && maximum_tokens.to_string() == "None" {
        abort_required_path_argument(
            VALIDATION_LABEL,
            &EXPECTED_KEYS,
            field_ident,
            attribute.span(),
        );
    }
    (minimum_tokens, maximum_tokens)
}

fn update_limit(
    field_ident: &syn::Ident,
    name_value: &syn::MetaNameValue,
    minimum: &mut Option<NumericInfo>,
    exclusive_minimum: &mut Option<NumericInfo>,
    maximum: &mut Option<NumericInfo>,
    exclusive_maximum: &mut Option<NumericInfo>,
) {
    let syn::MetaNameValue { path, lit, .. } = name_value;
    let path_ident = SingleIdentPath::new(path).ident();
    match path_ident.to_string().as_ref() {
        "minimum" => {
            update_numeric(minimum, field_ident, lit, path_ident);
        }
        "exclusive_minimum" => update_numeric(exclusive_minimum, field_ident, lit, path_ident),
        "maximum" => update_numeric(maximum, field_ident, lit, path_ident),
        "exclusive_maximum" => update_numeric(exclusive_maximum, field_ident, lit, path_ident),
        unknown_value => {
            abort_unknown_name_value_argument(
                VALIDATION_LABEL,
                unknown_value,
                &EXPECTED_KEYS,
                field_ident,
                path.span(),
                name_value,
            );
        }
    }
}

fn update_numeric(
    target: &mut Option<NumericInfo>,
    field_ident: &syn::Ident,
    lit: &syn::Lit,
    path_ident: &syn::Ident,
) {
    if target.is_some() {
        abort_duplicated_argument(VALIDATION_LABEL, field_ident, lit.span(), path_ident)
    }

    *target = Some(NumericInfo::new(
        get_numeric(VALIDATION_LABEL, field_ident, lit),
        path_ident.to_owned(),
    ));
}

fn get_limit_tokens(
    field_ident: &syn::Ident,
    inclusive_limit: Option<NumericInfo>,
    exclusive_limit: Option<NumericInfo>,
) -> proc_macro2::TokenStream {
    match (inclusive_limit, exclusive_limit) {
        (Some(inclusive), Some(exclusive)) => {
            let span = inclusive
                .path_ident()
                .span()
                .join(exclusive.path_ident().span())
                .unwrap_or(inclusive.path_ident().span());
            abort_invalid_attribute_on_field(
                field_ident,
                span,
                &format!(
                    "Both `{}` and `{}` have been set in `range` validator: conflict",
                    inclusive.path_name(),
                    exclusive.path_name()
                ),
            )
        }
        (Some(inclusive_limit), None) => {
            quote!(Some(::serde_valid::Limit::Inclusive(#inclusive_limit)))
        }
        (None, Some(exclusive_limit)) => {
            quote!(Some(::serde_valid::Limit::Exclusive(#exclusive_limit)))
        }
        (None, None) => quote!(None),
    }
}
