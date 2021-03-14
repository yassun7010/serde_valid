use crate::helper::NamedField;
use crate::validator::abort_invalid_attribute_on_field;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_length_validator(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Validator {
    if let Some(array_field) = field.array_field() {
        match array_field.ty() {
            syn::Type::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    if ["u8", "char"].contains(&format!("{}", ident).as_str()) {
                        return Validator::Normal(inner_extract_length_validator(
                            field.ident(),
                            attribute,
                            meta_items,
                        ));
                    }
                }
            }
            _ => (),
        }
        Validator::Array(Box::new(extract_length_validator(
            &array_field,
            attribute,
            meta_items,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_length_validator(
            &option_field,
            attribute,
            meta_items,
        )))
    } else {
        Validator::Normal(inner_extract_length_validator(
            field.ident(),
            attribute,
            meta_items,
        ))
    }
}

pub fn inner_extract_length_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> TokenStream {
    let mut min_length = None;
    let mut max_length = None;
    for meta_item in meta_items {
        if let syn::NestedMeta::Meta(ref item) = *meta_item {
            if let syn::Meta::NameValue(syn::MetaNameValue { ref path, lit, .. }) = item {
                let path_ident = path.get_ident().unwrap().to_owned();
                match path_ident.to_string().as_ref() {
                    "min_length" => {
                        min_length = Some(max_string(field_ident, lit, path_ident, min_length));
                    }
                    "max_length" => {
                        max_length = Some(max_string(field_ident, lit, path_ident, max_length));
                    }
                    v => abort_invalid_attribute_on_field(
                        field_ident,
                        path.span(),
                        &format!(
                            "unknown argument `{}` for validator `length` \
                            (it only has `min_length` or `max_length`)",
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
    let min_length_tokens = get_length_tokens(min_length);
    let max_length_tokens = get_length_tokens(max_length);

    if min_length_tokens.to_string() == "None" && max_length_tokens.to_string() == "None" {
        abort_invalid_attribute_on_field(
            field_ident,
            attribute.span(),
            "Validator `length` requires at least 1 argument from `min_length` or `max_length`",
        );
    }
    let token = quote!(
        if !::serde_valid::validate_length(
            #field_ident,
            #min_length_tokens,
            #max_length_tokens
        ) {
            errors.push(::serde_valid::Error::LengthError);
        }
    );
    token
}

fn max_string(
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
             &format!("invalid argument type for `{}` of `length` validator: only string literals are allowed", path_ident.to_string())),
    }
}

fn get_length_tokens(length: Option<syn::LitInt>) -> proc_macro2::TokenStream {
    match length {
        Some(value) => quote!(Some(#value)),
        None => quote!(None),
    }
}
