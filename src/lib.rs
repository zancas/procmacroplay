use proc_macro::TokenStream;

#[proc_macro]
pub fn token_for_if(_: TokenStream) -> TokenStream {
    "if".parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
