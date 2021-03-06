mod meta_list;
mod name_value;
use crate::abort::{abort_invalid_attribute_on_field, abort_unnamed_fields_struct};
use meta_list::extract_validator_from_meta_list;
use name_value::extract_validator_from_name_value;
use proc_macro_error::abort;
use syn::{parse_quote, spanned::Spanned};

/// Find the types (as string) for each field of the struct
/// Needed for the `must_match` filter
pub fn collect_validators(fields: &syn::Fields) -> Vec<proc_macro2::TokenStream> {
    let mut validators = vec![];
    for field in fields {
        let field_ident = &field
            .ident
            .as_ref()
            .unwrap_or_else(|| abort_unnamed_fields_struct(field.span()));
        for attribute in &field.attrs {
            if attribute.path != parse_quote!(validate) {
                continue;
            }
            let validator = extract_validator(field_ident, attribute);
            match validator {
                Some(validator) => validators.push(validator),
                None => abort_invalid_attribute_on_field(
                    &field_ident,
                    attribute.span(),
                    "it needs at least one validator",
                ),
            }
        }
    }

    validators
}

fn extract_validator(
    field_ident: &syn::Ident,
    attribute: &syn::Attribute,
) -> Option<proc_macro2::TokenStream> {
    match attribute.parse_meta() {
        Ok(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
            // only validation from there on
            for meta_item in nested {
                match meta_item {
                    syn::NestedMeta::Meta(item) => match item {
                        syn::Meta::List(meta_list) => {
                            return extract_validator_from_meta_list(
                                field_ident,
                                attribute,
                                meta_list,
                            )
                        }
                        syn::Meta::NameValue(name_value) => {
                            return extract_validator_from_name_value(
                                field_ident,
                                attribute,
                                name_value,
                            )
                        }
                        _ => abort!(item.span(), "unsupport non MetaList Meta type"),
                    },
                    _ => unreachable!("Found a non Meta while looking for validators"),
                };
            }
        }
        // TODO
        Ok(syn::Meta::Path(_)) => abort!(attribute.span(), "Non-support nested arguments"),
        Ok(syn::Meta::NameValue(_)) => {
            abort!(attribute.span(), "Unexpected name=value argument")
        }
        Err(e) => unreachable!(
            "Got something other than a list of attributes while checking field `{}`: {:?}",
            field_ident, e
        ),
    };
    None
}
