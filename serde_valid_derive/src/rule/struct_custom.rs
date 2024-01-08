use proc_macro2::TokenStream;
use quote::quote;

use crate::types::{CommaSeparatedNestedMetas, SingleIdentPath};
use crate::validate::{
    MetaListStructValidation, MetaNameValueStructValidation, MetaPathStructValidation, Validator,
};
use std::str::FromStr;

pub fn collect_struct_custom_from_named_struct(
    attributes: &[syn::Attribute],
) -> Result<TokenStream, crate::Errors> {
    let mut errors = vec![];

    let validations = attributes
        .iter()
        .filter_map(|attribute| {
            if attribute.path().is_ident("validate") {
                match extract_struct_validator(attribute) {
                    Ok(validator) => Some(validator),
                    Err(validator_error) => {
                        errors.extend(validator_error);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if errors.is_empty() {
        Ok(TokenStream::from_iter(validations))
    } else {
        Err(errors)
    }
}

fn extract_struct_validator(attribute: &syn::Attribute) -> Result<Validator, crate::Errors> {
    match &attribute.meta {
        syn::Meta::Path(_) => Ok(quote!()),
        syn::Meta::List(list) => extract_struct_validator_from_meta_list(attribute, list),
        syn::Meta::NameValue(name_value) => {
            Err(vec![crate::Error::validate_meta_name_value_not_support(
                name_value,
            )])
        }
    }
}

fn extract_struct_validator_from_meta_list(
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Result<TokenStream, crate::Errors> {
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

#[inline]
fn extract_struct_validator_from_nested_meta_path(
    validation_type: MetaPathStructValidation,
    _validation: &syn::Path,
) -> Result<Validator, crate::Errors> {
    match validation_type {}
}

fn extract_struct_validator_from_nested_meta_list(
    validation_type: MetaListStructValidation,
    validation: &syn::MetaList,
) -> Result<Validator, crate::Errors> {
    match validation_type {
        MetaListStructValidation::Custom => extract_struct_custom_validator(validation),
    }
}

#[inline]
fn extract_struct_validator_from_nested_meta_name_value(
    validation_type: MetaNameValueStructValidation,
    _validation: &syn::MetaNameValue,
) -> Result<Validator, crate::Errors> {
    match validation_type {}
}

fn extract_struct_custom_validator(meta_path: &syn::MetaList) -> Result<Validator, crate::Errors> {
    let mut errors = vec![];

    let nested = meta_path
        .parse_args_with(CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::rule_args_parse_error(meta_path, &error)])?;

    match nested.len() {
        0 => Err(vec![
            crate::Error::validate_custom_need_function_or_closure(meta_path),
        ])?,
        2.. => nested
            .iter()
            .skip(1)
            .for_each(|error| errors.push(crate::Error::rule_allow_single_function(error))),
        _ => {}
    }

    let rule = match &nested[0] {
        crate::types::NestedMeta::Meta(syn::Meta::Path(path)) => {
            extract_struct_custom_from_meta_path(path)
        }
        crate::types::NestedMeta::Closure(closure) => extract_struct_custom_from_closure(closure),
        _ => Err(vec![
            crate::Error::validate_custom_need_function_or_closure(&nested[0]),
        ]),
    };

    match rule {
        Ok(_) => {
            if errors.is_empty() {
                rule
            } else {
                Err(errors)
            }
        }
        Err(rule_errors) => Err(errors.into_iter().chain(rule_errors).collect()),
    }
}

fn extract_struct_custom_from_meta_path(meta_path: &syn::Path) -> Result<Validator, crate::Errors> {
    let rule_fn_name = &meta_path;

    Ok(quote!(
        if let Err(__error) = #rule_fn_name(self) {
            __rule_vec_errors.push(__error);
        };
    ))
}

fn extract_struct_custom_from_closure(
    closure: &syn::ExprClosure,
) -> Result<Validator, crate::Errors> {
    Ok(quote!(
        if let Err(__error) = serde_valid::helpers::wrap_closure_validation(self, #closure) {
            __rule_vec_errors.push(__error);
        };
    ))
}
