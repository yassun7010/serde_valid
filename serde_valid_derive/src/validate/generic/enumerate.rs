use crate::serde::rename::RenameMap;
use crate::types::Field;
use crate::validate::common::CustomMessageToken;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

type Lits<'a> = syn::punctuated::Punctuated<&'a syn::Lit, syn::token::Comma>;

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
            use ::serde_valid::error::ToDefaultMessage;
            use ::serde_valid::validation::IntoError;

            #errors
                .entry(#rename)
                .or_default()
                .push(__composited_error_params.into_error_by(&#custom_message));
        }
    ))
}

fn get_enumerate(
    syn::MetaList { path, nested, .. }: &syn::MetaList,
) -> Result<Lits, crate::Errors> {
    let mut errors = vec![];
    let mut enumerate = Lits::new();

    if nested.is_empty() {
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
