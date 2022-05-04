use crate::abort::abort_invalid_attribute_on_field;
use crate::lit::LitNumeric;
use crate::types::Field;

pub fn get_numeric<'a>(lit: &'a syn::Lit) -> Result<LitNumeric<'a>, crate::Error> {
    match lit {
        syn::Lit::Int(int) => Ok(LitNumeric::Int(int)),
        syn::Lit::Float(float) => Ok(LitNumeric::Float(float)),
        _ => Err(crate::Error::new_numeric_literal_error(lit.span())),
    }
}

pub fn get_str<'a>(
    validation_label: &str,
    field: &impl Field,
    lit: &'a syn::Lit,
) -> &'a syn::LitStr {
    match lit {
        syn::Lit::Str(l) => l,
        _ => abort_invalid_attribute_on_field(
            field,
            lit.span(),
            &format!(
                "invalid argument type for `{}` validator: only str literals are allowed",
                validation_label
            ),
        ),
    }
}
