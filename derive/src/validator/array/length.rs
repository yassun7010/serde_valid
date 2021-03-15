use crate::helper::NamedField;
use crate::validator::abort_invalid_attribute_on_field;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_array_length_validator(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Validator {
    if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_array_length_validator(
            &option_field,
            attribute,
            meta_items,
        )))
    } else {
        Validator::Normal(inner_extract_array_length_validator(
            field.ident(),
            attribute,
            meta_items,
        ))
    }
}

pub fn inner_extract_array_length_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let mut min_items = None;
    let mut max_items = None;
    for meta_item in meta_items {
        if let syn::NestedMeta::Meta(ref item) = *meta_item {
            if let syn::Meta::NameValue(syn::MetaNameValue { ref path, lit, .. }) = item {
                let path_ident = path.get_ident().unwrap().to_owned();
                match path_ident.to_string().as_ref() {
                    "min_items" => {
                        min_items = Some(limit_int(field_ident, lit, path_ident, min_items));
                    }
                    "max_items" => {
                        max_items = Some(limit_int(field_ident, lit, path_ident, max_items));
                    }
                    v => abort_invalid_attribute_on_field(
                        field_ident,
                        path.span(),
                        &format!(
                            "unknown argument `{}` for validator `length` \
                            (it only has `min_items` or `max_items`)",
                            v
                        ),
                    ),
                }
            } else {
                abort_invalid_attribute_on_field(
                    field_ident,
                    item.span(),
                    &format!(
                        "unexpected item {:?} while parsing `length` validator of field {}",
                        item, field_ident
                    ),
                )
            }
        }
    }
    let min_items_tokens = get_length_tokens(min_items);
    let max_items_tokens = get_length_tokens(max_items);

    if min_items_tokens.to_string() == "None" && max_items_tokens.to_string() == "None" {
        abort_invalid_attribute_on_field(
            field_ident,
            attribute.span(),
            "Validator `length` requires at least 1 argument from `min_items` or `max_items`",
        );
    }
    let token = quote!(
        if !::serde_valid::validate_array_length(
            #field_ident,
            #min_items_tokens,
            #max_items_tokens
        ) {
            errors.push(::serde_valid::Error::ItemsError);
        }
    );
    token
}

fn limit_int(
    field_ident: &syn::Ident,
    lit: &syn::Lit,
    path_ident: syn::Ident,
    target: Option<syn::LitInt>,
) -> syn::LitInt {
    if target.is_some() {
        abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
            &format!(
                "duplicated `{}` argument of `length` validator: only unique argument is allowed",
                path_ident.to_string()
            ),
        )
    }

    match lit {
        syn::Lit::Int(l) => l.to_owned(),
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
             &format!("invalid argument type for `{}` of `length` validator: only int literals are allowed", path_ident.to_string())),
    }
}

fn get_length_tokens(length: Option<syn::LitInt>) -> proc_macro2::TokenStream {
    match length {
        Some(value) => quote!(Some(#value)),
        None => quote!(None),
    }
}
