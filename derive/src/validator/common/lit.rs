use crate::abort::abort_invalid_attribute_on_field;
use crate::lit::LitNumeric;
use crate::types::Field;

pub fn get_numeric<'a>(
    validation_label: &str,
    field: &impl Field,
    lit: &'a syn::Lit,
) -> LitNumeric<'a> {
    match lit {
        syn::Lit::Int(l) => LitNumeric::Int(l),
        syn::Lit::Float(l) => LitNumeric::Float(l),
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

pub fn get_integer<'a>(
    validation_label: &str,
    field: &impl Field,
    lit: &'a syn::Lit,
) -> &'a syn::LitInt {
    match lit {
        syn::Lit::Int(lit_int) => lit_int,
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
