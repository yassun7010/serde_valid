use crate::serde::rename::RenameMap;
use crate::types::Field;
use crate::validate::common::CustomMessageToken;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

type Lits<'a> = syn::punctuated::Punctuated<syn::Lit, syn::token::Comma>;

pub fn extract_generic_enumerate_validator(
    field: &impl Field,
    item_list: &syn::MetaList,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    inner_extract_generic_enumerate_validator(field, item_list, custom_message, rename_map)
}

fn inner_extract_generic_enumerate_validator(
    field: &impl Field,
    item_list: &syn::MetaList,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<TokenStream, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();
    let enumerate = get_enumerate(item_list)?;
    let custom_message = custom_message.into_token();

    Ok(quote!(
        if let Err(__composited_error_params) = ::serde_valid::validation::ValidateCompositedEnumerate::validate_composited_enumerate(
            #field_ident,
            &[#enumerate],
        ) {
            use ::serde_valid::validation::{IntoError, ToDefaultMessage};

            #errors
                .entry(#rename)
                .or_default()
                .push(__composited_error_params.into_error_by(#custom_message));
        }
    ))
}

fn get_enumerate(meta_list: &syn::MetaList) -> Result<Lits, crate::Errors> {
    let mut errors = vec![];
    let mut enumerate = Lits::new();
    let nested = meta_list
        .parse_args_with(crate::types::CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| {
            vec![crate::Error::validate_enumerate_parse_error(
                &meta_list.path,
                &error,
            )]
        })?;

    if nested.is_empty() {
        errors.push(crate::Error::validate_enumerate_need_item(&meta_list.path));
    }
    for item in nested {
        match &item {
            crate::types::NestedMeta::Lit(lit) => enumerate.push(lit.clone()),
            crate::types::NestedMeta::Meta(meta) => {
                errors.push(crate::Error::literal_only_from_meta(meta))
            }
        }
    }

    if errors.is_empty() {
        Ok(enumerate)
    } else {
        Err(errors)
    }
}
