use crate::serde::rename::RenameMap;
use crate::types::{Field, SingleIdentPath};
use crate::validate::common::{CustomMessageToken, MetaListValidation};
use crate::validate::generic::{
    extract_generic_custom_validator, extract_generic_enumerate_validator,
};
use crate::validate::{MetaNameValueValidation, MetaPathValidation, Validator};
use std::str::FromStr;

pub fn extract_validator_from_nested_meta_list(
    field: &impl Field,
    validation: &syn::MetaList,
    custom_message: CustomMessageToken,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let mut errors = vec![];

    let validation_ident = SingleIdentPath::new(&validation.path).ident();
    let validation_name = validation_ident.to_string();

    match MetaListValidation::from_str(&validation_name) {
        Ok(MetaListValidation::Enumerate) => {
            extract_generic_enumerate_validator(field, validation, custom_message, rename_map)
        }
        Ok(MetaListValidation::Custom) => {
            extract_generic_custom_validator(field, validation, rename_map)
        }
        Err(unknown) => {
            let error = if MetaNameValueValidation::from_str(&validation_name).is_ok() {
                crate::Error::validate_meta_name_value_need_value(
                    &validation.path,
                    &validation_name,
                )
            } else if MetaPathValidation::from_str(&validation_name).is_ok() {
                crate::Error::validate_meta_path_need_value(&validation.path, &validation_name)
            } else {
                crate::Error::validate_unknown_type(
                    &validation.path,
                    &unknown,
                    &(MetaPathValidation::iter().map(|x| x.name()))
                        .chain(MetaNameValueValidation::iter().map(|x| x.name()))
                        .chain(MetaListValidation::iter().map(|x| x.name()))
                        .collect::<Vec<_>>(),
                )
            };
            errors.push(error);

            Err(errors)
        }
    }
}
