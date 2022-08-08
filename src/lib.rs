extern crate proc_macro;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use xxhash_rust::xxh64::xxh64;

#[proc_macro]
pub fn name(input: TokenStream) -> TokenStream {
    let st = input.to_string();
    let hash = xxh64(st.as_bytes(), 0);
    return quote! {crate::namegen::Name(#hash)}.into()
}