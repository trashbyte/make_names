#![cfg_attr(not(feature="std"), no_std)]

extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::buffer::TokenBuffer;
use quote::quote;

#[cfg(feature="std")] use std::hash::{Hash, Hasher};

#[cfg(not(feature="std"))] extern crate alloc;
#[cfg(not(feature="std"))] use alloc::vec::Vec;
#[cfg(not(feature="std"))] use alloc::string::ToString;
#[cfg(not(feature="std"))] use core::hash::{Hash, Hasher};


#[proc_macro]
pub fn setup_names(input: TokenStream) -> TokenStream {
    let mut names = Vec::new();
    let mut name_idents = Vec::new();
    let mut hashes = Vec::new();

    let buffer = TokenBuffer::new(input);
    let mut cursor = buffer.begin();
    while !cursor.eof() {
        if let Some((ident, cur)) = cursor.ident() {
            let s = ident.to_string();
            let mut hasher = hashers::fnv::FNV1aHasher64::default();
            s.hash(&mut hasher);
            names.push(s.clone());
            name_idents.push(Ident::new(&s, Span::call_site()));
            hashes.push(hasher.finish());

            cursor = cur;

            if cursor.eof() { break; }

            if let Some((p, cur)) = cursor.punct() {
                if p.as_char() != ',' {
                    panic!("Expected comma-separated list of idents. Unexpected token: {}", p.as_char());
                }
                cursor = cur;
            }
            else {
                panic!("Expected comma-separated list of idents. Unexpected token: {}", cursor.token_tree().unwrap().0.to_string());
            }
        }
        else {
            panic!("Expected comma-separated list of idents. Unexpected token: {}", cursor.token_tree().unwrap().0.to_string());
        }
    }

    let retval = quote! {
use std::collections::hash_map::DefaultHasher;
use std::hash::BuildHasherDefault;
use hashbrown::HashMap;

#(pub const #name_idents: u64 = #hashes;)*

lazy_static!{
    pub static ref hashes_to_names: HashMap<u64, String, BuildHasherDefault<DefaultHasher>> = {
        let mut map = HashMap::default();
        #(map.insert(#hashes, #names .to_string());)*
        map
    };
}

pub fn to_string(hash: u64) -> String {
    hashes_to_names.get(&hash).unwrap().clone()
}
    };
    retval.into()
}
