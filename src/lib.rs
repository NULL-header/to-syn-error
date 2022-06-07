use proc_macro2::Span;
pub use to_syn_error_derive::*;

pub trait ToSynError {
    fn to_syn_error(&self, span: Span) -> syn::Error;
}
