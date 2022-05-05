use crate::lit::LitNumeric;

pub fn get_numeric<'a>(lit: &'a syn::Lit) -> Result<LitNumeric<'a>, crate::Error> {
    match lit {
        syn::Lit::Int(int) => Ok(LitNumeric::Int(int)),
        syn::Lit::Float(float) => Ok(LitNumeric::Float(float)),
        _ => Err(crate::Error::new_numeric_literal_error(lit)),
    }
}

pub fn get_str(lit: &syn::Lit) -> Result<&syn::LitStr, crate::Error> {
    match lit {
        syn::Lit::Str(lit_str) => Ok(lit_str),
        _ => Err(crate::Error::new_str_literal_error(lit)),
    }
}
