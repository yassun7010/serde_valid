use crate::helper::SingleIdentPath;
use crate::validator::abort_invalid_attribute_on_field;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_message_tokens(
    field_ident: &syn::Ident,
    _attribute: &syn::Attribute,
    meta_items: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> Option<TokenStream> {
    let mut message_fmt = None;
    for meta_item in meta_items {
        if let syn::NestedMeta::Meta(ref item) = *meta_item {
            match item {
                syn::Meta::List(meta_list) => {
                    update_message_fn_from_meta_list(&mut message_fmt, field_ident, meta_list)
                }
                _ => continue,
            }
        }
    }
    message_fmt
}

fn update_message_fn_from_meta_list(
    message_fn: &mut Option<TokenStream>,
    field_ident: &syn::Ident,
    syn::MetaList { path, nested, .. }: &syn::MetaList,
) {
    let ident = SingleIdentPath::new(&path).ident();

    match ident.to_string().as_ref() {
        "message_fn" => return update_message_fn_from_nested(message_fn, field_ident, nested),
        _ => {}
    }
}

fn update_message_fn_from_nested(
    message_fn: &mut Option<TokenStream>,
    field_ident: &syn::Ident,
    nested: &syn::punctuated::Punctuated<syn::NestedMeta, syn::Token![,]>,
) {
    for meta_item in nested {
        if let syn::NestedMeta::Meta(ref item) = *meta_item {
            match item {
                syn::Meta::Path(path) => {
                    abort_duplicated_message_fn_argument(message_fn, field_ident, nested);
                    *message_fn = Some(quote!(#path));
                }
                _ => abort_invalid_attribute_on_field(
                    field_ident,
                    item.span(),
                    &format!(
                        "Unexpected item {:?} while parsing `message_fn` of field {}",
                        item, field_ident
                    ),
                ),
            }
        }
    }
}

fn abort_duplicated_message_fn_argument(
    message_fn: &mut Option<TokenStream>,
    field_ident: &syn::Ident,
    nested: &syn::punctuated::Punctuated<syn::NestedMeta, syn::Token![,]>,
) {
    if message_fn.is_some() {
        abort_invalid_attribute_on_field(
            field_ident,
            nested.span(),
            &format!(
                "Duplicated `message_fn` argument of field {}: only unique argument is allowed",
                field_ident,
            ),
        )
    }
}
