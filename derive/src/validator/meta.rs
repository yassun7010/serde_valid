mod meta_path;
mod nested_meta_list;
mod nested_meta_name_value;
mod nested_meta_path;

use crate::errors::invalid_field_attribute_error;
use crate::types::Field;
use crate::validator::Validator;
use meta_path::extract_validator_from_meta_path;
use nested_meta_list::extract_validator_from_nested_meta_list;
use nested_meta_name_value::extract_validator_from_nested_meta_name_value;
use nested_meta_path::extract_validator_from_nested_meta_path;
use proc_macro_error::abort;
use syn::spanned::Spanned;

pub fn extract_meta_validator(
    field: &impl Field,
    attribute: &syn::Attribute,
) -> Result<Validator, Vec<syn::Error>> {
    match attribute.parse_meta() {
        Ok(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
            // only validation from there on
            for meta_item in nested {
                match meta_item {
                    syn::NestedMeta::Meta(item) => match item {
                        syn::Meta::Path(validation) => {
                            return extract_validator_from_nested_meta_path(
                                field, attribute, validation,
                            )
                        }
                        syn::Meta::List(validation_list) => {
                            return extract_validator_from_nested_meta_list(
                                field,
                                attribute,
                                validation_list,
                            )
                        }
                        syn::Meta::NameValue(validation_name_value) => {
                            return extract_validator_from_nested_meta_name_value(
                                field,
                                attribute,
                                validation_name_value,
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
        Ok(syn::Meta::Path(validation)) => {
            return extract_validator_from_meta_path(field, attribute, &validation)
        }
        Ok(syn::Meta::NameValue(_)) => {
            abort!(attribute.span(), "Unexpected name=value argument")
        }
        Err(err) => abort!(
            attribute.span(),
            "Got something other than a list of attributes while checking field `{}`: {:?}",
            field.ident(),
            err
        ),
    };

    Err(vec![invalid_field_attribute_error(
        field,
        attribute.span(),
        "it needs at least one validator",
    )])
}
