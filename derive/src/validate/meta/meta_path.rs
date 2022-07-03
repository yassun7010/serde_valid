use std::collections::HashMap;

use crate::types::Field;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_validator_from_meta_path(
    field: &impl Field,
    rename_map: &HashMap<String, String>,
) -> Result<Validator, crate::Errors> {
    Ok(inner_extract_validator_from_meta_path(field, rename_map))
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
                __object_errors @ ::serde_valid::validation::Errors::Object(_) => {
                    __properties_errors.insert(
                        #rename,
                        vec![::serde_valid::validation::Error::Properties(__object_errors)]
                    );
                }
                __array_errors @ ::serde_valid::validation::Errors::Array(_) => {
                    unimplemented!();
                }
                ::serde_valid::validation::Errors::NewType(__new_type_errors) => {
                    __properties_errors.insert(#rename, __new_type_errors);
                }
            }
        }
    )
}
