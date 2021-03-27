use crate::abort::abort_invalid_attribute_on_field;
use crate::helper::SingleIdentPath;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_length_validator_tokens(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
    validation_label: &str,
    min_label: &str,
    max_label: &str,
) -> (TokenStream, TokenStream) {
    let mut min = None;
    let mut max = None;
    for meta_item in meta_items {
        if let syn::NestedMeta::Meta(ref item) = *meta_item {
            if let syn::Meta::NameValue(syn::MetaNameValue { path, lit, .. }) = item {
                let path_ident = SingleIdentPath::new(&path).ident();
                let path_str = path_ident.to_string();
                if path_str == min_label {
                    min = Some(limit_int(
                        field_ident,
                        lit,
                        path_ident,
                        min,
                        validation_label,
                    ));
                } else if path_str == max_label {
                    max = Some(limit_int(
                        field_ident,
                        lit,
                        path_ident,
                        max,
                        validation_label,
                    ));
                } else {
                    abort_invalid_attribute_on_field(
                        field_ident,
                        path.span(),
                        &format!(
                            "Unknown argument `{}` for validator `{}` \
                            (it only has `{}` or `{}`)",
                            path_str, validation_label, min_label, max_label
                        ),
                    )
                }
            } else {
                abort_invalid_attribute_on_field(
                    field_ident,
                    item.span(),
                    &format!(
                        "Unexpected item {:?} while parsing `{}` validator of field {}",
                        item, validation_label, field_ident
                    ),
                )
            }
        }
    }
    let min_tokens = get_limit_tokens(min);
    let max_tokens = get_limit_tokens(max);

    if min_tokens.to_string() == "None" && max_tokens.to_string() == "None" {
        abort_invalid_attribute_on_field(
            field_ident,
            attribute.span(),
            &format!(
                "Validator `{}` requires at least 1 argument from `{}` or `{}`",
                validation_label, min_label, max_label
            ),
        );
    }
    (min_tokens, max_tokens)
}

fn limit_int(
    field_ident: &syn::Ident,
    lit: &syn::Lit,
    path_ident: &syn::Ident,
    target: Option<syn::LitInt>,
    validation_label: &str,
) -> syn::LitInt {
    if target.is_some() {
        abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
            &format!(
                "Duplicated `{}` argument of `{}` validator: only unique argument is allowed",
                path_ident.to_string(),
                validation_label
            ),
        )
    }

    match lit {
        syn::Lit::Int(l) => l.to_owned(),
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
            &format!(
                "invalid argument type for `{}` of `{}` validator: only int literals are allowed",
                path_ident.to_string(),
                validation_label
            ),
        ),
    }
}

fn get_limit_tokens(limit: Option<syn::LitInt>) -> proc_macro2::TokenStream {
    match limit {
        Some(value) => quote!(Some(#value)),
        None => quote!(None),
    }
}
