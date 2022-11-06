use super::nested_meta_list::extract_validator_from_nested_meta_list;
use super::nested_meta_name_value::extract_validator_from_nested_meta_name_value;
use super::nested_meta_path::extract_validator_from_nested_meta_path;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use crate::validate::common::{extract_custom_message_tokens, CustomMessageToken};
use crate::validate::Validator;

pub fn extract_validator_from_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    syn::MetaList { nested, .. }: &syn::MetaList,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let mut errors = vec![];
    let custom_message = match nested.len() {
        0..=1 => CustomMessageToken::default(),
        2 => match extract_custom_message_tokens(&nested[1]) {
            Ok(custom_message) => custom_message,
            Err(message_fn_errors) => {
                errors.extend(message_fn_errors);
                CustomMessageToken::default()
            }
        },
        _ => {
            for meta in nested.iter().skip(1) {
                errors.push(crate::Error::too_many_list_items(meta));
            }
            CustomMessageToken::default()
        }
    };

    if !nested.is_empty() {
        let meta_item = &nested[0];
        match meta_item {
            syn::NestedMeta::Meta(meta) => {
                let validator = match meta {
                    syn::Meta::Path(path) => extract_validator_from_nested_meta_path(
                        field,
                        path,
                        custom_message,
                        rename_map,
                    ),
                    syn::Meta::List(list) => extract_validator_from_nested_meta_list(
                        field,
                        list,
                        custom_message,
                        rename_map,
                    ),
                    syn::Meta::NameValue(name_value) => {
                        extract_validator_from_nested_meta_name_value(
                            field,
                            attribute,
                            name_value,
                            custom_message,
                            rename_map,
                        )
                    }
                };
                match validator {
                    Ok(validator) => {
                        if errors.is_empty() {
                            Ok(validator)
                        } else {
                            Err(errors)
                        }
                    }
                    Err(validator_errors) => {
                        errors.extend(validator_errors);
                        Err(errors)
                    }
                }
            }
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
