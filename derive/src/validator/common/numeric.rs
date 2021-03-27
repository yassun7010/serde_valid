use crate::abort::abort_invalid_attribute_on_field;
use crate::lit::LitNumeric;

pub fn get_numeric(validation_label: &str, field_ident: &syn::Ident, lit: &syn::Lit) -> LitNumeric {
    match lit {
        syn::Lit::Int(l) => LitNumeric::Int(l.to_owned()),
        syn::Lit::Float(l) => LitNumeric::Float(l.to_owned()),
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
            &format!(
                "invalid argument type for `{}` validator: only numeric literals are allowed",
                validation_label
            ),
        ),
    }
}

pub fn get_integer(
    validation_label: &str,
    field_ident: &syn::Ident,
    lit: &syn::Lit,
) -> syn::LitInt {
    match lit {
        syn::Lit::Int(lit_int) => lit_int.to_owned(),
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
            &format!(
                "invalid argument type for `{}` validator: only integer literals are allowed",
                validation_label
            ),
        ),
    }
}
