use crate::field_validate::Validator;

use crate::field_validate::{
    MetaListStructValidation, MetaNameValueStructValidation, MetaPathStructValidation,
};
use crate::types::SingleIdentPath;
use std::str::FromStr;

use super::nested_meta_list::extract_struct_validator_from_nested_meta_list;
use super::nested_meta_name_value::extract_struct_validator_from_nested_meta_name_value;
use super::nested_meta_path::extract_struct_validator_from_nested_meta_path;

pub fn extract_struct_validator_from_meta_list(
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Result<Validator, crate::Errors> {
    let mut errors = vec![];
    let nested = meta_list
        .parse_args_with(crate::types::CommaSeparatedMetas::parse_terminated)
        .map_err(|error| {
            vec![crate::Error::validate_attribute_parse_error(
                attribute, &error,
            )]
        })?;

    match nested.len() {
        0 => Err(vec![crate::Error::struct_validation_type_required(
            attribute,
        )])?,
        1 => {}
        _ => {
            for meta in nested.iter().skip(2) {
                errors.push(crate::Error::too_many_list_items(meta));
            }
        }
    };

    let meta = &nested[0];
    let validation_path = match meta {
        syn::Meta::Path(path) => path,
        syn::Meta::List(list) => &list.path,
        syn::Meta::NameValue(name_value) => &name_value.path,
    };

    let validation_name = SingleIdentPath::new(validation_path).ident().to_string();
    let validator = match (
        MetaPathStructValidation::from_str(&validation_name),
        MetaListStructValidation::from_str(&validation_name),
        MetaNameValueStructValidation::from_str(&validation_name),
        meta,
    ) {
        (Ok(validation_type), _, _, syn::Meta::Path(validation)) => {
            extract_struct_validator_from_nested_meta_path(validation_type, validation)
        }

        (_, Ok(validation_type), _, syn::Meta::List(validation)) => {
            extract_struct_validator_from_nested_meta_list(validation_type, validation)
        }

        (_, _, Ok(validation_type), syn::Meta::NameValue(validation)) => {
            extract_struct_validator_from_nested_meta_name_value(validation_type, validation)
        }

        (Ok(_), _, _, _) => Err(vec![crate::Error::meta_path_validation_need_value(
            validation_path,
            &validation_name,
        )]),

        (_, Ok(_), _, _) => Err(vec![crate::Error::meta_list_validation_need_value(
            validation_path,
            &validation_name,
        )]),

        (_, _, Ok(_), _) => Err(vec![crate::Error::meta_name_value_validation_need_value(
            validation_path,
            &validation_name,
        )]),

        _ => Err(vec![crate::Error::struct_validation_type_unknown(
            validation_path,
            &validation_name,
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
}
