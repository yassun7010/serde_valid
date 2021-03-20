mod nested_meta_list;
mod nested_meta_name_value;
mod nested_meta_path;

use super::nested::extract_nested_validator;
use crate::helper::NamedField;
use crate::validator::Validator;
use nested_meta_list::extract_validator_from_nested_meta_list;
use nested_meta_name_value::extract_validator_from_nested_meta_name_value;
use nested_meta_path::extract_validator_from_nested_meta_path;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_meta_validator(field: &NamedField, attribute: &syn::Attribute) -> Option<Validator> {
    match attribute.parse_meta() {
        Ok(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
            // only validation from there on
            for meta_item in nested {
                match meta_item {
                    syn::NestedMeta::Meta(item) => match item {
                        syn::Meta::Path(path) => {
                            return extract_validator_from_nested_meta_path(field, attribute, path)
                        }
                        syn::Meta::List(meta_list) => {
                            return extract_validator_from_nested_meta_list(
                                field, attribute, meta_list,
                            )
                        }
                        syn::Meta::NameValue(name_value) => {
                            return extract_validator_from_nested_meta_name_value(
                                field, attribute, name_value,
                            )
                        }
                    },
                    _ => abort!(
                        meta_item.span(),
                        "Found a non Meta while looking for validators"
                    ),
                };
            }
        }
        Ok(syn::Meta::Path(path)) => return extract_nested_validator(field, attribute, &path),
        Ok(syn::Meta::NameValue(_)) => {
            abort!(attribute.span(), "Unexpected name=value argument")
        }
        Err(e) => abort!(
            attribute.span(),
            "Got something other than a list of attributes while checking field `{}`: {:?}",
            field.ident(),
            e
        ),
    };
    None
}
