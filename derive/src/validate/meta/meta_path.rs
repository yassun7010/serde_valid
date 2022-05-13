use std::collections::HashMap;

use crate::types::Field;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_validator_from_meta_path(
    field: &impl Field,
    rename_map: &HashMap<String, String>,
) -> Result<Validator, crate::Errors> {
    let validator = if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_validator_from_meta_path(
            &array_field,
            rename_map,
        )?))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_validator_from_meta_path(
            &option_field,
            rename_map,
        )?))
    } else {
        Validator::Normal(inner_extract_validator_from_meta_path(field, rename_map))
    };

    Ok(validator)
}

fn inner_extract_validator_from_meta_path(
    field: &impl Field,
    rename_map: &HashMap<String, String>,
) -> TokenStream {
    let field_ident = field.ident();
    let field_name = field.name();
    let rename = rename_map.get(field_name).unwrap_or(field_name);

    quote!(
        if let Err(__inner_errors) = #field_ident.validate() {
            match __inner_errors {
                __fields_errors @ ::serde_valid::validation::Errors::Fields(_) => {
                    __errors.insert(
                        #rename,
                        vec![::serde_valid::validation::Error::Nested(__fields_errors)]
                    );
                }
                ::serde_valid::validation::Errors::NewType(__new_type_errors) => {
                    __errors.insert(#rename, __new_type_errors);
                }
            }
        }
    )
}
