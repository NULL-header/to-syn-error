pub trait ToSynError {
    fn to_syn_error() -> syn::Error;
}

#[cfg(test)]
mod test {
    #[test]
    fn tmp() {
        use thiserror::Error;
        use to_syn_error_derive::ToSynError;
        #[derive(Error, Debug, ToSynError)]
        enum Test {}
        panic!("holder");
    }
}