use super::nested_meta_list::extract_validator_from_nested_meta_list;
use super::nested_meta_name_value::extract_validator_from_nested_meta_name_value;
use super::nested_meta_path::extract_validator_from_nested_meta_path;
use crate::types::Field;
use crate::validator::common::extract_message_fn_tokens;
use crate::validator::Validator;
use syn::spanned::Spanned;

pub fn extract_validator_from_meta_list(
    field: &impl Field,
    attribute: &syn::Attribute,
    syn::MetaList { nested, .. }: &syn::MetaList,
) -> Result<Validator, crate::Error> {
    let messaeg_fn = match nested.len() {
        0..=1 => None,
        2 => Some(extract_message_fn_tokens(&nested[1])?),
        _ => Err(crate::Error::too_many_list_items(nested[2].span()))?,
    };

    let validation = if nested.len() > 0 {
        let meta_item = &nested[0];
        match meta_item {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(path) => {
                    extract_validator_from_nested_meta_path(field, path, messaeg_fn)
                }
                syn::Meta::List(list) => {
                    extract_validator_from_nested_meta_list(field, attribute, list, messaeg_fn)
                }
                syn::Meta::NameValue(name_value) => extract_validator_from_nested_meta_name_value(
                    field, attribute, name_value, messaeg_fn,
                ),
            },
            syn::NestedMeta::Lit(lit) => {
                Err(crate::Error::validate_meta_literal_not_support(lit.span()))
            }
        }
    } else {
        Err(crate::Error::new_attribute_required_error(attribute.span()))
    };

    validation
}
