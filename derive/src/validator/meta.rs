mod meta_list;
mod name_value;

use crate::helper::NamedField;
use crate::validator::Validator;
use meta_list::extract_validator_from_meta_list;
use name_value::extract_validator_from_name_value;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_meta_validator(field: &NamedField, attribute: &syn::Attribute) -> Option<Validator> {
    match attribute.parse_meta() {
        Ok(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
            // only validation from there on
            for meta_item in nested {
                match meta_item {
                    syn::NestedMeta::Meta(item) => match item {
                        syn::Meta::List(meta_list) => {
                            return extract_validator_from_meta_list(field, attribute, meta_list)
                        }
                        syn::Meta::NameValue(name_value) => {
                            return extract_validator_from_name_value(field, attribute, name_value)
                        }
                        _ => abort!(item.span(), "unsupport non MetaList Meta type"),
                    },
                    _ => unreachable!("Found a non Meta while looking for validators"),
                };
            }
        }
        // TODO
        Ok(syn::Meta::Path(_)) => abort!(attribute.span(), "Non-support nested arguments"),
        Ok(syn::Meta::NameValue(_)) => {
            abort!(attribute.span(), "Unexpected name=value argument")
        }
        Err(e) => unreachable!(
            "Got something other than a list of attributes while checking field `{}`: {:?}",
            field.ident(),
            e
        ),
    };
    None
}
