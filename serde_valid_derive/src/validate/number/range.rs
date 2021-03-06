use crate::lit::{LitNumber, Number};
use proc_macro2::TokenStream;
use crate::validate::abort_invalid_attribute_on_field;
use syn::spanned::Spanned;
use quote::quote;

pub fn extract_range_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
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
                let path_ident = path.get_ident().unwrap().to_owned();
                match path_ident.to_string().as_ref() {
                        "minimum" => {
                            minimum = Some(get_number(field_ident, lit, path_ident, minimum));
                        },
                        "exclusive_minimum" => {
                            exclusive_minimum = Some(get_number(field_ident, lit, path_ident, exclusive_minimum));
                        },
                        "maximum" => {
                            maximum = Some(get_number(field_ident, lit, path_ident, maximum));
                        },
                        "exclusive_maximum" => {
                            exclusive_maximum = Some(get_number(field_ident, lit, path_ident, exclusive_maximum));
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
            "Validator `range` requires at least 1 argument out of `minimum` or `exclusive_minimum`, `maximum` or `exclusive_maximum`",
        );
    }
    let validator_param = quote!(self.#field_ident);
    quote!(
        if !::serde_valid::validate_range(
            #validator_param,
            #minimum_tokens,
            #maximum_tokens
        ) {
            errors.push(::serde_valid::Error::RangeError);
        }
    )
}

fn get_number(field_ident: &syn::Ident, lit: &syn::Lit, path_ident: syn::Ident, target: Option<Number>) -> Number {
    if target.is_some() {
        abort_invalid_attribute_on_field(
            field_ident,
                lit.span(),
                &format!("duplicated `{}` argument of `range` validator: only unique argument is allowed", path_ident.to_string()))
    }

    match lit {
        syn::Lit::Int(l) => Number::new(LitNumber::Int(l.to_owned()), path_ident),
        syn::Lit::Float(l) => Number::new(LitNumber::Float(l.to_owned()), path_ident), 
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
             &format!("invalid argument type for `{}` of `range` validator: only number literals are allowed", path_ident.to_string())),
    }
}

fn get_limit_tokens(field_ident: &syn::Ident, inclusive_limit: Option<Number>, exclusive_limit: Option<Number>) -> proc_macro2::TokenStream {
    match (inclusive_limit, exclusive_limit) {
        (Some(inclusive), Some(exclusive)) => abort_invalid_attribute_on_field(
            field_ident,
            inclusive.path_ident().span().join(exclusive.path_ident().span()).unwrap(),
            &format!("both `{}` and `{}` have been set in `range` validator: conflict", inclusive.path_name(), exclusive.path_name())
        ),
        (Some(inclusive_limit), None) => quote!(Some(::serde_valid::Limit::Inclusive(#inclusive_limit))),
        (None, Some(exclusive_limit)) => quote!(Some(::serde_valid::Limit::Exclusive(#exclusive_limit))),
        (None, None) => quote!(None)
    }
}
