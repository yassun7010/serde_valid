pub mod minimum_duration;

use std::str::FromStr;
use proc_macro2::TokenStream;
use quote::quote;
pub use minimum_duration::extract_string_minimum_duration_validator;
use crate::attribute::common::lit::get_str;
use crate::error::Error;

fn extract_duration(value: &syn::Lit) -> Result<TokenStream, crate::Errors> {
    let pattern = get_str(value)?.value();
    if let Some(index) = pattern.find(char::is_alphabetic) {
        let test = &pattern[index..];
        let number = u64::from_str(&pattern[..index]).map_err(|_| crate::Errors::new())?;
        match test {
            "ns" => {
                Ok(
                    quote!(
                            let duration = Duration::from_nanos(#number);
                        )
                )
            }
            "ms" => {
                Ok(
                    quote!(
                            let duration = Duration::from_millis(#number);
                        )
                )
            }
            "s" => {
                Ok(
                    quote!(
                            let duration = Duration::from_secs(#number);
                        )
                )
            }
            _ => {
                Err(vec![Error::duration_str_wrong_suffix(&value, &["ns", "ms", "s"])])
            }
        }
    } else { Err(vec![Error::duration_str_no_suffix(&value)]) }
}