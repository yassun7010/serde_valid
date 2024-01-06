use crate::serde::rename::RenameMap;
use crate::types::{CommaSeparatedNestedMetas, Field, SingleIdentPath};
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_generic_custom_validator(
    field: &impl Field,
    meta_list: &syn::MetaList,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let path = &meta_list.path;
    let path_ident = SingleIdentPath::new(path).ident();
    let field_name = field.name();
    let field_ident = field.ident();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();
    let nested = meta_list
        .parse_args_with(CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::message_fn_parse_error(path_ident, &error)])?;

    let custom_fn_name = match nested.len() {
        0 => Err(crate::Error::validate_custom_need_item(path)),
        1 => extract_custom_fn_name(&nested[0]),
        _ => Err(crate::Error::validate_custom_tail_error(&nested)),
    }
    .map_err(|error| vec![error])?;

    Ok(quote!(
        if let Err(__error) = #custom_fn_name(#field_ident) {
            #errors
                .entry(#rename)
                .or_default()
                .push(__error);
        };
    ))
}

fn extract_custom_fn_name(
    nested_meta: &crate::types::NestedMeta,
) -> Result<TokenStream, crate::Error> {
    match nested_meta {
        crate::types::NestedMeta::Meta(meta) => match meta {
            syn::Meta::List(list) => {
                let fn_name = &list.path;
                Ok(quote!(#fn_name))
            }
            syn::Meta::NameValue(name_value) => {
                Err(crate::Error::meta_name_value_not_support(name_value))
            }
            syn::Meta::Path(fn_name) => Ok(quote!(#fn_name)),
        },
        crate::types::NestedMeta::Lit(lit) => Err(crate::Error::literal_not_support(lit)),
    }
}
