use proc_macro::TokenStream;

#[proc_macro]
pub fn token_for_if(_: TokenStream) -> TokenStream {
    "if".parse().unwrap()
}
