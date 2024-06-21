use syn::spanned::Spanned;

use crate::{
    attribute::{
        common::message_format::MessageFormat,
        struct_validate::generic::extract_generic_struct_custom_validator_from_meta_list,
        MetaListStructValidation, Validator,
    },
    warning::{Warning, WithWarnings},
};

pub fn extract_struct_validator_from_meta_list(
    validation_type: MetaListStructValidation,
    validation: &syn::MetaList,
    message_format: MessageFormat,
) -> Result<WithWarnings<Validator>, crate::Errors> {
    match validation_type {
        MetaListStructValidation::Custom => {
            extract_generic_struct_custom_validator_from_meta_list(validation, message_format).map(
                |validator| {
                    WithWarnings::new_with_warnings(
                        validator,
                        vec![Warning::new_custom_meta_list_deprecated(
                            validation.path.get_ident().unwrap(),
                            validation.span(),
                        )],
                    )
                },
            )
        }
    }
}
