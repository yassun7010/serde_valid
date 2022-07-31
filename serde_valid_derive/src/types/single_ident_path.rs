use proc_macro_error::abort;
use syn::spanned::Spanned;

pub struct SingleIdentPath<'a>(&'a syn::Path);

impl<'a> SingleIdentPath<'a> {
    pub fn new(path: &'a syn::Path) -> Self {
        if path.get_ident().is_none() {
            abort!(
                path.span(),
                "Path(='{}') must be single ident path.",
                path_to_string(path)
            )
        }
        Self(path)
    }

    pub fn ident(&self) -> &'a syn::Ident {
        self.0.get_ident().unwrap()
    }
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .pairs()
        .map(|pair| match pair {
            syn::punctuated::Pair::Punctuated(seg, ..) => {
                format!("{}::", seg.ident)
            }
            syn::punctuated::Pair::End(seg) => seg.ident.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}
