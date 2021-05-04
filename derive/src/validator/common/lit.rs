use crate::abort::abort_invalid_attribute_on_field;
use crate::lit::LitNumeric;
use crate::types::Field;

pub fn get_numeric(validation_label: &str, field: &impl Field, lit: &syn::Lit) -> LitNumeric {
    match lit {
        syn::Lit::Int(l) => LitNumeric::Int(l.to_owned()),
        syn::Lit::Float(l) => LitNumeric::Float(l.to_owned()),
        _ => abort_invalid_attribute_on_field(
            field,
            lit.span(),
            &format!(
                "invalid argument type for `{}` validator: only numeric literals are allowed",
                validation_label
            ),
        ),
    }
}

pub fn get_integer(validation_label: &str, field: &impl Field, lit: &syn::Lit) -> syn::LitInt {
    match lit {
        syn::Lit::Int(lit_int) => lit_int.to_owned(),
        _ => abort_invalid_attribute_on_field(
            field,
            lit.span(),
            &format!(
                "invalid argument type for `{}` validator: only integer literals are allowed",
                validation_label
            ),
        ),
    }
}

pub fn get_str(validation_label: &str, field: &impl Field, lit: &syn::Lit) -> syn::LitStr {
    match lit {
        syn::Lit::Str(l) => l.to_owned(),
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
