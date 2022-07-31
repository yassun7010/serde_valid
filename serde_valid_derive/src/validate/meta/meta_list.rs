use super::nested_meta_list::extract_validator_from_nested_meta_list;
use super::nested_meta_name_value::extract_validator_from_nested_meta_name_value;
use super::nested_meta_path::extract_validator_from_nested_meta_path;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use crate::validate::common::extract_message_fn_tokens;
use crate::validate::Validator;

pub fn extract_validator_from_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    syn::MetaList { nested, .. }: &syn::MetaList,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let mut errors = vec![];
    let messaeg_fn = match nested.len() {
        0..=1 => None,
        2 => match extract_message_fn_tokens(&nested[1]) {
            Ok(message_fn) => Some(message_fn),
            Err(message_fn_errors) => {
                errors.extend(message_fn_errors);
                None
            }
        },
        _ => {
            for meta in nested.iter().skip(1) {
                errors.push(crate::Error::too_many_list_items(meta));
            }
            None
        }
    };

    if nested.len() > 0 {
        let meta_item = &nested[0];
        match meta_item {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(path) => {
                    extract_validator_from_nested_meta_path(field, path, messaeg_fn, rename_map)
                }
                syn::Meta::List(list) => {
                    extract_validator_from_nested_meta_list(field, list, messaeg_fn, rename_map)
                }
                syn::Meta::NameValue(name_value) => extract_validator_from_nested_meta_name_value(
                    field, attribute, name_value, messaeg_fn, rename_map,
                ),
            }
            .map_err(|validator_errors| {
                errors.extend(validator_errors);
                errors
            }),
            syn::NestedMeta::Lit(lit) => {
                errors.push(crate::Error::validate_meta_literal_not_support(lit));
                Err(errors)
            }
        }
    } else {
        errors.push(crate::Error::validate_type_required_error(attribute));
        Err(errors)
    }
}
