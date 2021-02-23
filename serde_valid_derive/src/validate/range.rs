
use proc_macro2::TokenStream;
use crate::validate::abort_invalid_attribute_on_field;
use syn::{spanned::Spanned};
use quote::{quote, ToTokens};

pub enum LitNumber{
    Int(syn::LitInt),
    Float(syn::LitFloat)
}

impl LitNumber {
    fn span(&self) -> proc_macro2::Span {
        match self {
            Self::Int(lin) => lin.span(),
            Self::Float(lin) => lin.span(),
        }
    }
}

impl ToTokens for LitNumber {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Int(lin) => lin.to_tokens(tokens),
            Self::Float(lin) => lin.to_tokens(tokens)
        }
    }
}

fn get_number(field_ident: &syn::Ident, lit: &syn::Lit, name: &str, target: Option<LitNumber>) -> LitNumber {
    if target.is_some() {
        abort_invalid_attribute_on_field(
            field_ident,
                lit.span(),
                &format!("duplicated `{}` argument of `range` validator: only unique argument is allowed", name))
    }

    match lit {
        syn::Lit::Int(l) => LitNumber::Int(l.to_owned()),
        syn::Lit::Float(l) => LitNumber::Float(l.to_owned()), 
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
             &format!("invalid argument type for `{}` of `range` validator: only number literals are allowed", name)),
    }
}

pub fn extract_range_validator(
    field_ident: &syn::Ident,
    attr: &syn::Attribute,
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
                let ident = path.get_ident().unwrap();
                match ident.to_string().as_ref() {
                        "minimum" => {
                            minimum = Some(get_number(field_ident, lit, "minimum", minimum));
                        },
                        "exclusive_minimum" => {
                            exclusive_minimum = Some(get_number(field_ident, lit, "exclusive_minimum", exclusive_minimum));
                        },
                        "maximum" => {
                            maximum = Some(get_number(field_ident, lit, "maximum", maximum));
                        },
                        "exclusive_maximum" => {
                            exclusive_maximum = Some(get_number(field_ident, lit, "exclusive_maximum", exclusive_maximum));
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
    let minimum_tokens= match (minimum, exclusive_minimum) {
        (Some(_), Some(lit)) => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(), 
            "both `minimum` and `exclusive_minimum` have been set in `range` validator: conflict"
        ),
        (Some(minimum), None) => quote!(Some(::serde_valid::Limit::Inclusive(#minimum))),
        (None, Some(exclusive_minimum)) => quote!(Some(::serde_valid::Limit::Exclusive(#exclusive_minimum))),
        (None, None) => quote!(None)
    };
    let maximum_tokens = match (maximum, exclusive_maximum) {
        (Some(_), Some(lit)) => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(), 
            "both `maximum` and `exclusive_maximum` have been set in `range` validator: conflict"
        ),
        (Some(maximum), None) => quote!(Some(::serde_valid::Limit::Inclusive(#maximum))),
        (None, Some(exclusive_maximum)) => quote!(Some(::serde_valid::Limit::Exclusive(#exclusive_maximum))),
        (None, None) => quote!(None)
    };
    
    if minimum_tokens.to_string() == "None" && maximum_tokens.to_string() == "None" {
        abort_invalid_attribute_on_field(
            field_ident,
            attr.span(),
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
