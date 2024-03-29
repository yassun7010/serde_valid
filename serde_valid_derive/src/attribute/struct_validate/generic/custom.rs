use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::Validator;
use crate::types::CommaSeparatedNestedMetas;
use quote::quote;

pub fn extract_generic_struct_custom_validator(
    meta_list: &syn::MetaList,
    _message_format: MessageFormat,
) -> Result<Validator, crate::Errors> {
    let mut errors = vec![];

    let nested = meta_list
        .parse_args_with(CommaSeparatedNestedMetas::parse_terminated)
        .map_err(|error| vec![crate::Error::rule_args_parse_error(meta_list, &error)])?;

    match nested.len() {
        0 => Err(vec![
            crate::Error::validate_custom_need_function_or_closure(meta_list),
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
