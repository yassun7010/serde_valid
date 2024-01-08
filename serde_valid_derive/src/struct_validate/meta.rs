mod meta_list;
mod nested_meta_list;
mod nested_meta_name_value;
mod nested_meta_path;

use quote::quote;

use crate::field_validate::Validator;

pub fn extract_struct_validator(attribute: &syn::Attribute) -> Result<Validator, crate::Errors> {
    match &attribute.meta {
        syn::Meta::Path(_) => Ok(quote!()),
        syn::Meta::List(list) => {
            meta_list::extract_struct_validator_from_meta_list(attribute, list)
        }
        syn::Meta::NameValue(name_value) => {
            Err(vec![crate::Error::validate_meta_name_value_not_support(
                name_value,
            )])
        }
    }
}
