use crate::lit::{LitNumeric, NumericInfo};
use crate::helper::{NamedField, SingleIdentPath};
use crate::validator::abort_invalid_attribute_on_field;
use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use quote::quote;
use crate::validator::Validator;

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
        Validator::Option(
            Box::new(extract_numeric_range_validator(
            &option_field,
                attribute,
                meta_items,
        )))
    } else{
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
    let (minimum_tokens, maximum_tokens) = extract_numeric_range_validator_tokens(
        field_ident,
        attribute,
        meta_items,
    );
    quote!(
        if !::serde_valid::validate_numeric_range(
            *#field_ident,
            #minimum_tokens,
            #maximum_tokens
        ) {
            errors.push(
                ::serde_valid::Error::RangeError(
                    ::serde_valid::error::Message::new(
                        #field_string,
                        ::serde_valid::error::RangeErrorInfo::new(
                            *#field_ident,
                            #minimum_tokens,
                            #maximum_tokens
                        )
                    )
                )
            );
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
            if let syn::Meta::NameValue(syn::MetaNameValue {
                ref path, lit, ..
            }) = item
            {
                let path_ident = SingleIdentPath::new(path).ident();
                match path_ident.to_string().as_ref() {
                        "minimum" => {
                            minimum = Some(get_numeric(field_ident, lit, path_ident, minimum));
                        },
                        "exclusive_minimum" => {
                            exclusive_minimum = Some(get_numeric(field_ident, lit, path_ident, exclusive_minimum));
                        },
                        "maximum" => {
                            maximum = Some(get_numeric(field_ident, lit, path_ident, maximum));
                        },
                        "exclusive_maximum" => {
                            exclusive_maximum = Some(get_numeric(field_ident, lit, path_ident, exclusive_maximum));
                        },
                        v => abort_invalid_attribute_on_field(field_ident, path.span(), &format!(
                            "unknown argument `{}` for validator `range` \
                            (it only has `minimum` or `exclusive_minimum`, \
                            `maximum` or `exclusive_maximum`)",
                            v
                        ))
                    }
            } else {
                abort_invalid_attribute_on_field(
                    field_ident,
                    item.span(),
                    &format!(
                        "unexpected item {:?} while parsing `range` validator of field {}",
                        item, field_ident
                    ),
                )
            }
        }
        
    }
    let minimum_tokens = get_limit_tokens(
        field_ident,  minimum,  exclusive_minimum
    );
    let maximum_tokens = get_limit_tokens(
        field_ident,  maximum,  exclusive_maximum
    );
    
    if minimum_tokens.to_string() == "None" && maximum_tokens.to_string() == "None" {
        abort_invalid_attribute_on_field(
            field_ident,
            attribute.span(),
            "Validator `range` requires at least 1 argument from `minimum` or `exclusive_minimum`, `maximum` or `exclusive_maximum`",
        );
    }
    (minimum_tokens, maximum_tokens)
}

fn get_numeric(field_ident: &syn::Ident, lit: &syn::Lit, path_ident: &syn::Ident, target: Option<NumericInfo>) -> NumericInfo {
    if target.is_some() {
        abort_invalid_attribute_on_field(
            field_ident,
                lit.span(),
                &format!("duplicated `{}` argument of `range` validator: only unique argument is allowed", path_ident.to_string()))
    }

    match lit {
        syn::Lit::Int(l) => NumericInfo::new(LitNumeric::Int(l.to_owned()), path_ident.to_owned()),
        syn::Lit::Float(l) => NumericInfo::new(LitNumeric::Float(l.to_owned()), path_ident.to_owned()), 
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
             &format!("invalid argument type for `{}` of `range` validator: only numeric literals are allowed", path_ident.to_string())),
    }
}

fn get_limit_tokens(field_ident: &syn::Ident, inclusive_limit: Option<NumericInfo>, exclusive_limit: Option<NumericInfo>) -> proc_macro2::TokenStream {
    match (inclusive_limit, exclusive_limit) {
        (Some(inclusive), Some(exclusive)) => abort_invalid_attribute_on_field(
            field_ident,
            inclusive.path_ident().span()
            .join(
                exclusive.path_ident().span()
            )
            .unwrap_or(inclusive.path_ident().span()),
            &format!("both `{}` and `{}` have been set in `range` validator: conflict", inclusive.path_name(), exclusive.path_name())
        ),
        (Some(inclusive_limit), None) => quote!(Some(::serde_valid::Limit::Inclusive(#inclusive_limit))),
        (None, Some(exclusive_limit)) => quote!(Some(::serde_valid::Limit::Exclusive(#exclusive_limit))),
        (None, None) => quote!(None)
    }
}
