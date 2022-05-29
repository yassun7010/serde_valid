use std::collections::HashMap;

use crate::types::Field;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

type Lits<'a> = syn::punctuated::Punctuated<&'a syn::Lit, syn::token::Comma>;

pub fn extract_generic_enumerate_validator(
    field: &impl Field,
    item_list: &syn::MetaList,
    message_fn: Option<TokenStream>,
    rename_map: &HashMap<String, String>,
) -> Result<Validator, crate::Errors> {
    Ok(inner_extract_generic_enumerate_validator(
        field, item_list, message_fn, rename_map,
    )?)
}

fn inner_extract_generic_enumerate_validator(
    field: &impl Field,
    item_list: &syn::MetaList,
    message_fn: Option<TokenStream>,
    rename_map: &HashMap<String, String>,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let rename = rename_map.get(field_name).unwrap_or(field_name);
    let enumerate = get_enumerate(item_list)?;
    let message = message_fn.unwrap_or(quote!(
        ::serde_valid::EnumerateErrorParams::to_default_message
    ));

    Ok(quote!(
        if let Err(error_params) = ::serde_valid::ValidateEnumerate::validate_enumerate(
            #field_ident,
            &[#enumerate],
        ) {
            use ::serde_valid::error::ToDefaultMessage;
            __errors
                .entry(#rename)
                .or_default()
                .push(::serde_valid::validation::Error::Enumerate(
                    ::serde_valid::error::Message::new(
                        error_params,
                        #message
                )
                ));
        }
    ))
}

fn get_enumerate<'a>(
    syn::MetaList { path, nested, .. }: &'a syn::MetaList,
) -> Result<Lits<'a>, crate::Errors> {
    let mut errors = vec![];
    let mut enumerate = Lits::new();

    if nested.len() == 0 {
        errors.push(crate::Error::validate_enumerate_need_item(path));
    }
    for item in nested {
        match item {
            syn::NestedMeta::Lit(lit) => enumerate.push(lit),
            syn::NestedMeta::Meta(meta) => errors.push(crate::Error::literal_only(meta)),
        }
    }

    if errors.is_empty() {
        Ok(enumerate)
    } else {
        Err(errors)
    }
}
