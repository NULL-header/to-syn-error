use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use thiserror::Error;

#[derive(Debug, Error)]
enum InputError {
    #[error("The macro needs an enum.")]
    NotEnum,
}

impl InputError {
    fn to_syn_error(&self, span: Span) -> syn::Error {
        syn::Error::new(span, self.to_string())
    }
}

pub struct Input(syn::ItemEnum);

impl Input {
    pub fn get_name(&self) -> &syn::Ident {
        &self.0.ident
    }
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item_enum: syn::ItemEnum = match input.parse() {
            Ok(i) => i,
            Err(e) => {
                return Err(InputError::NotEnum.to_syn_error(e.span()));
            }
        };
        Ok(Self(item_enum))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use assert_parse::{make_assert, register_assert};
    use assert_parse::register_assert;
    use quote::quote;
    use rstest::*;

    register_assert!(Input, InputError);

    #[rstest]
    fn not_enum(assert: Assert) {
        let input = quote! {
            struct Mock;
        };
        assert.error(input, InputError::NotEnum);
    }
}
