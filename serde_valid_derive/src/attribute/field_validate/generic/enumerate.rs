use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use quote::quote;

type Lits<'a> = syn::punctuated::Punctuated<syn::Lit, syn::token::Comma>;

pub fn extract_generic_enumerate_validator(
    field: &impl Field,
    item_list: &syn::MetaList,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    inner_extract_generic_enumerate_validator(field, item_list, message_format, rename_map)
}

fn inner_extract_generic_enumerate_validator(
    field: &impl Field,
    item_list: &syn::MetaList,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let field_name = field.name();
    let field_ident = field.ident();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();
    let enumerate = get_enumerate(item_list)?;

    Ok(quote!(
        if let Err(__composited_error_params) = ::serde_valid::validation::ValidateCompositedEnumerate::validate_composited_enumerate(
            #field_ident,
            &[#enumerate],
        ) {
            use ::serde_valid::validation::IntoError;
            use ::serde_valid::validation::error::FormatDefault;

            #errors
                .entry(#rename)
                .or_default()
                .push(__composited_error_params.into_error_by(#message_format));
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
                meta_list, &error,
            )]
        })?;

    if nested.is_empty() {
        errors.push(crate::Error::validate_enumerate_need_item(&meta_list.path));
    }
    for item in nested {
        match &item {
            crate::types::NestedMeta::Lit(lit) => enumerate.push(lit.clone()),
            crate::types::NestedMeta::Meta(meta) => errors.push(crate::Error::literal_only(meta)),
            crate::types::NestedMeta::Closure(closure) => {
                errors.push(crate::Error::closure_not_supported(closure))
            }
        }
    }

    if errors.is_empty() {
        Ok(enumerate)
    } else {
        Err(errors)
    }
}
