use proc_macro_error::abort;
use syn::spanned::Spanned;
use syn::{GenericArgument, Path, PathArguments};

pub fn extract_element_type_from_array(ty: &syn::Type) -> Option<syn::Type> {
    match *ty {
        syn::Type::Path(ref typepath) if typepath.qself.is_none() => {
            extract_element_type_from_vec(&typepath.path, ty)
        }
        syn::Type::Array(ref array) => Some(*array.elem.to_owned()),
        _ => None,
    }
}

fn extract_element_type_from_vec(path: &Path, ty: &syn::Type) -> Option<syn::Type> {
    let idents_of_path = path
        .segments
        .iter()
        .into_iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<String>>()
        .join("::");
    if ["Vec", "vec::Vec", "std::vec::Vec", "alloc::vec::Vec"].contains(&idents_of_path.as_str()) {
        path.segments
            .last()
            .and_then(|path_segment| {
                let type_params = &path_segment.arguments;
                match *type_params {
                    PathArguments::AngleBracketed(ref params) => params.args.first(),
                    _ => abort!(ty.span(), "`Vec` must be angle bracketed (=`Vec<*>`)."),
                }
            })
            .and_then(|generic_arg| match *generic_arg {
                GenericArgument::Type(ref ty) => Some(ty.to_owned()),
                _ => None,
            })
    } else {
        None
    }
}

pub fn make_element_ident(ident: &syn::Ident, span: proc_macro2::Span) -> syn::Ident {
    syn::Ident::new(
        &format!("_elem_{}", &ident.to_string().trim_start_matches("_")),
        span,
    )
}

pub fn make_element_field(
    field: &syn::Field,
    span: proc_macro2::Span,
    inner_ty: syn::Type,
) -> syn::Field {
    let inner_ident = field
        .ident
        .as_ref()
        .map(|ident| make_element_ident(ident, span));
    syn::Field {
        attrs: field.attrs.to_owned(),
        vis: field.vis.to_owned(),
        ident: inner_ident,
        colon_token: field.colon_token,
        ty: inner_ty,
    }
}
