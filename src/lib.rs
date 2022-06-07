use proc_macro2::Span;

pub trait ToSynError {
    fn to_syn_error(&self, span: Span) -> syn::Error;
}
