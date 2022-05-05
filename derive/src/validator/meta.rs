mod meta_path;
mod nested_meta_list;
mod nested_meta_name_value;
mod nested_meta_path;

use crate::types::Field;
use crate::validator::Validator;
use meta_path::extract_validator_from_meta_path;
use nested_meta_list::extract_validator_from_nested_meta_list;
use nested_meta_name_value::extract_validator_from_nested_meta_name_value;
use nested_meta_path::extract_validator_from_nested_meta_path;
use syn::spanned::Spanned;

use super::common::extract_message_fn_tokens;

pub fn extract_meta_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
) -> Result<Validator, crate::Error> {
    match attribute.parse_meta() {
        Ok(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
            let messaeg_fn = match nested.len() {
                0..=1 => None,
                2 => Some(extract_message_fn_tokens(&nested[1])?),
                _ => Err(crate::Error::too_many_list_items(nested[2].span()))?,
            };

            let validation = if nested.len() > 0 {
                let meta_item = &nested[0];
                match meta_item {
                    syn::NestedMeta::Meta(meta) => match meta {
                        syn::Meta::Path(path) => extract_validator_from_nested_meta_path(
                            field, attribute, path, messaeg_fn,
                        ),
                        syn::Meta::List(list) => extract_validator_from_nested_meta_list(
                            field, attribute, list, messaeg_fn,
                        ),
                        syn::Meta::NameValue(name_value) => {
                            extract_validator_from_nested_meta_name_value(
                                field, attribute, name_value, messaeg_fn,
                            )
                        }
                    },
                    syn::NestedMeta::Lit(lit) => {
                        Err(crate::Error::new_literal_meta_item_error(lit.span()))
                    }
                }
            } else {
                Err(crate::Error::new_attribute_required_error(attribute.span()))
            };
            return validation;
        }
        Ok(syn::Meta::Path(_)) => return extract_validator_from_meta_path(field),
        Ok(syn::Meta::NameValue(_)) => {
            return Err(crate::Error::new_meta_name_value_item_error(
                attribute.span(),
            ))
        }
        Err(error) => {
            return Err(crate::Error::new_attribute_parse_error(
                attribute.span(),
                &error,
            ))
        }
    };
}
