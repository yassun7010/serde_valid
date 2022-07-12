use crate::types::Field;
use crate::validate::common::get_numeric;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
macro_rules! extract_numeric_range_validator{
    ($ErrorType:ident) => {
        paste::paste! {
            pub fn [<extract_numeric_ $ErrorType:snake _validator>](
                field: &impl Field,
                validation_value: &syn::Lit,
                message_fn: Option<TokenStream>,
                rename_map: &HashMap<String, String>,
            ) -> Result<Validator, crate::Errors> {
                Ok([<inner_extract_numeric_ $ErrorType:snake _validator>](field, validation_value, message_fn, rename_map)?)
            }

            fn [<inner_extract_numeric_ $ErrorType:snake _validator>](
                field: &impl Field,
                validation_value: &syn::Lit,
                message_fn: Option<TokenStream>,
                rename_map: &HashMap<String, String>,
            ) -> Result<TokenStream, crate::Errors> {
                let field_name = field.name();
                let field_ident = field.ident();
                let rename = rename_map.get(field_name).unwrap_or(field_name);
                let [<$ErrorType:snake>] = get_numeric(validation_value)?;
                let message =
                    message_fn.unwrap_or(quote!(::serde_valid::[<$ErrorType ErrorParams>]::to_default_message));

                Ok(quote!(
                    if let Err(__composited_error_params) = ::serde_valid::validation::[<ValidateComposited $ErrorType>]::[<validate_composited_ $ErrorType:snake>](
                        #field_ident,
                        #[<$ErrorType:snake>],
                    ) {
                        use ::serde_valid::error::ToDefaultMessage;
                        use ::serde_valid::validation::IntoError;

                        __properties_errors
                            .entry(#rename)
                            .or_default()
                            .push(__composited_error_params.into_error_by(#message)
                        );
                    }
                ))
            }
        }
    }
}

extract_numeric_range_validator!(Maximum);
extract_numeric_range_validator!(Minimum);
extract_numeric_range_validator!(ExclusiveMaximum);
extract_numeric_range_validator!(ExclusiveMinimum);
