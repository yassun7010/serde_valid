use crate::helper::NamedField;
use crate::validator::abort_invalid_attribute_on_field;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_object_size_validator(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_object_size_validator(
            &array_field,
            attribute,
            meta_items,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_object_size_validator(
            &option_field,
            attribute,
            meta_items,
        )))
    } else {
        Validator::Normal(inner_extract_object_size_validator(
            field.ident(),
            attribute,
            meta_items,
        ))
    }
}

pub fn inner_extract_object_size_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let mut min_properties = None;
    let mut max_properties = None;
    for meta_item in meta_items {
        if let syn::NestedMeta::Meta(ref item) = *meta_item {
            if let syn::Meta::NameValue(syn::MetaNameValue { ref path, lit, .. }) = item {
                let path_ident = path.get_ident().unwrap().to_owned();
                match path_ident.to_string().as_ref() {
                    "min_properties" => {
                        min_properties =
                            Some(limit_int(field_ident, lit, path_ident, min_properties));
                    }
                    "max_properties" => {
                        max_properties =
                            Some(limit_int(field_ident, lit, path_ident, max_properties));
                    }
                    v => abort_invalid_attribute_on_field(
                        field_ident,
                        path.span(),
                        &format!(
                            "unknown argument `{}` for validator `properties` \
                            (it only has `min_properties` or `max_properties`)",
                            v
                        ),
                    ),
                }
            } else {
                abort_invalid_attribute_on_field(
                    field_ident,
                    item.span(),
                    &format!(
                        "unexpected item {:?} while parsing `properties` validator of field {}",
                        item, field_ident
                    ),
                )
            }
        }
    }
    let min_properties_tokens = get_length_tokens(min_properties);
    let max_properties_tokens = get_length_tokens(max_properties);

    if min_properties_tokens.to_string() == "None" && max_properties_tokens.to_string() == "None" {
        abort_invalid_attribute_on_field(
            field_ident,
            attribute.span(),
            "Validator `properties` requires at least 1 argument from `min_properties` or `max_properties`",
        );
    }
    let token = quote!(
        if !::serde_valid::validate_object_size(
            #field_ident,
            #min_properties_tokens,
            #max_properties_tokens
        ) {
            errors.push(::serde_valid::Error::PropertiesError);
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
                "duplicated `{}` argument of `properties` validator: only unique argument is allowed",
                path_ident.to_string()
            ),
        )
    }

    match lit {
        syn::Lit::Int(l) => l.to_owned(),
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
             &format!("invalid argument type for `{}` of `properties` validator: only int literals are allowed", path_ident.to_string())),
    }
}

fn get_length_tokens(length: Option<syn::LitInt>) -> proc_macro2::TokenStream {
    match length {
        Some(value) => quote!(Some(#value)),
        None => quote!(None),
    }
}
