extern crate proc_macro;

mod parse;

use parse::Input;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ToSynError)]
pub fn to_syn_error_derive(input: TokenStream) -> TokenStream {
    let input: Input = match syn::parse(input) {
        Ok(i) => i,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };
    let name = input.get_name();
    quote! {
        impl ToSynError for #name{
            fn to_syn_error(&self,span:proc_macro2::Span)->syn::Error{
                syn::Error::new(span,self.to_string())
            }
        }
    }
    .into()
}
