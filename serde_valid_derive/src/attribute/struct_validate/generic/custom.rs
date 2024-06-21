use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use crate::types::CommaSeparatedNestedMetas;
use quote::quote;

pub fn extract_generic_struct_custom_validator_from_meta_list(
    meta_list: &syn::MetaList,
    _message_format: MessageFormat,
) -> Result<Validator, crate::Errors> {
    let mut errors = vec![];

    let nested = meta_list
        .parse_args_with(CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::rule_args_parse_error(meta_list, &error)])?;

    match nested.len() {
        0 => Err(vec![
            crate::Error::validate_custom_meta_list_need_function_or_closure(meta_list),
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
        crate::types::NestedMeta::Meta(syn::Meta::List(list)) => {
            extract_struct_custom_from_meta_list(list)
        }
        crate::types::NestedMeta::Closure(closure) => extract_struct_custom_from_closure(closure),
        _ => Err(vec![
            crate::Error::validate_custom_meta_list_need_function_or_closure(&nested[0]),
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

pub fn extract_generic_struct_custom_validator_from_meta_name_value(
    meta_name_value: &syn::MetaNameValue,
    _message_format: MessageFormat,
) -> Result<Validator, crate::Errors> {
    match &meta_name_value.value {
        syn::Expr::Path(syn::ExprPath { path, .. }) => extract_struct_custom_from_meta_path(path),
        syn::Expr::Call(call) => extract_struct_custom_from_call(call),
        syn::Expr::Closure(closure) => extract_struct_custom_from_closure(closure),
        _ => Err(vec![
            crate::Error::validate_custom_meta_name_value_need_function_or_closure(meta_name_value),
        ]),
    }
}

fn extract_struct_custom_from_meta_path(meta_path: &syn::Path) -> Result<Validator, crate::Errors> {
    let rule_fn_name = &meta_path;

    Ok(quote!(
        if let Err(__errors) = serde_valid::validation::custom::wrap_into_vec_errors(#rule_fn_name(self)) {
            __rule_vec_errors.extend(__errors);
        };
    ))
}

fn extract_struct_custom_from_meta_list(
    meta_list: &syn::MetaList,
) -> Result<Validator, crate::Errors> {
    Ok(quote!(
        if let Err(__errors) = serde_valid::validation::custom::wrap_closure_validation(self, #meta_list) {
            __rule_vec_errors.extend(__errors);
        };
    ))
}

fn extract_struct_custom_from_call(call: &syn::ExprCall) -> Result<Validator, crate::Errors> {
    Ok(quote!(
        if let Err(__errors) = serde_valid::validation::custom::wrap_call_validation(self, #call) {
            __rule_vec_errors.extend(__errors);
        };
    ))
}

fn extract_struct_custom_from_closure(
    closure: &syn::ExprClosure,
) -> Result<Validator, crate::Errors> {
    Ok(quote!(
        if let Err(__errors) = serde_valid::validation::custom::wrap_closure_validation(self, #closure) {
            __rule_vec_errors.extend(__errors);
        };
    ))
}
