use super::nested_meta_list::extract_validator_from_nested_meta_list;
use super::nested_meta_name_value::extract_validator_from_nested_meta_name_value;
use super::nested_meta_path::extract_validator_from_nested_meta_path;
use crate::serde::rename::RenameMap;
use crate::types::{Field, SingleIdentPath};
use crate::validate::common::{extract_custom_message_tokens, CustomMessageToken};
use crate::validate::{MetaListValidation, MetaNameValueValidation, MetaPathValidation, Validator};
use std::str::FromStr;

pub fn extract_validator_from_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let mut errors = vec![];
    let nested = meta_list
        .parse_args_with(crate::types::CommaSeparatedMetas::parse_terminated)
        .map_err(|error| {
            vec![crate::Error::validate_attribute_parse_error(
                attribute, &error,
            )]
        })?;

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
        let meta = &nested[0];

        let validation_path = match meta {
            syn::Meta::Path(path) => path,
            syn::Meta::List(list) => &list.path,
            syn::Meta::NameValue(name_value) => &name_value.path,
        };

        let validation_name = SingleIdentPath::new(validation_path).ident().to_string();

        let validator = match (
            MetaPathValidation::from_str(&validation_name),
            MetaListValidation::from_str(&validation_name),
            MetaNameValueValidation::from_str(&validation_name),
            meta,
        ) {
            (Ok(validation_type), _, _, syn::Meta::Path(path)) => {
                extract_validator_from_nested_meta_path(
                    field,
                    validation_type,
                    path,
                    custom_message,
                    rename_map,
                )
            }
            (_, Ok(validation_type), _, syn::Meta::List(list)) => {
                extract_validator_from_nested_meta_list(
                    field,
                    validation_type,
                    list,
                    custom_message,
                    rename_map,
                )
            }
            (_, _, Ok(validation_type), syn::Meta::NameValue(name_value)) => {
                extract_validator_from_nested_meta_name_value(
                    field,
                    validation_type,
                    name_value,
                    custom_message,
                    rename_map,
                )
            }
            (Ok(_), _, _, _) => Err(vec![crate::Error::validate_meta_path_need_value(
                validation_path,
                &validation_name,
            )]),
            (_, Ok(_), _, _) => Err(vec![crate::Error::validate_meta_list_need_value(
                validation_path,
                &validation_name,
            )]),
            (_, _, Ok(_), _) => Err(vec![crate::Error::validate_meta_name_value_need_value(
                validation_path,
                &validation_name,
            )]),
            _ => Err(vec![crate::Error::validate_unknown_type(
                validation_path,
                &validation_name,
                &(MetaPathValidation::iter().map(|x| x.name()))
                    .chain(MetaListValidation::iter().map(|x| x.name()))
                    .chain(MetaNameValueValidation::iter().map(|x| x.name()))
                    .collect::<Vec<_>>(),
            )]),
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
    } else {
        errors.push(crate::Error::validate_type_required_error(attribute));
        Err(errors)
    }
}
