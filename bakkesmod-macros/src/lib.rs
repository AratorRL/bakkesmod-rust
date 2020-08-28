#![allow(unused)]
extern crate proc_macro;
use proc_macro2::{Ident, Span};

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn plugin_init(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut parsed_input: syn::ItemFn = syn::parse_macro_input!(input as syn::ItemFn);
    let name = parsed_input.clone().sig.ident;

    let tokens = quote! {
        #parsed_input

        #[no_mangle]
        pub extern "C" fn InitPlugin(id: u64) {
            bakkesmod::bakkesmod_init(id);
            #name();
        }

        #[no_mangle]
        pub extern "C" fn ExitPlugin() {
            bakkesmod::bakkesmod_exit();
        }
    };

    tokens.into()
}