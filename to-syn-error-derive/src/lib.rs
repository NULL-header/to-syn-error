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
    dbg!(input);
    quote! {}.into()
}
