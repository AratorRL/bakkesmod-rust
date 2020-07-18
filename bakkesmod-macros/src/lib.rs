#![allow(unused)]
extern crate proc_macro;
use proc_macro2::{Ident, Span};

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn plugin_init(attr: TokenStream, input: TokenStream) -> TokenStream {
    // let attr = attr.into();
    // let input = input.into();

    // let item = syn::parse2::<syn::Item>(input)?;
    // let opts = syn::parse2(attr)?;

    let mut parsed_input: syn::ItemFn = syn::parse_macro_input!(input as syn::ItemFn);

    // let fn_item: syn::Item = syn::parse(input).unwrap();
    // let fn_ident = input.ident.clone();
    // let name = &parsed_input.ident.to_string();
    // println!("name = {}", name);
    // let func_call = quote! { #name };
    // let ident = Ident::new(name, Span::call_site());

    let name = parsed_input.clone().sig.ident;

    let tokens = quote! {
        #parsed_input

        #[no_mangle]
        pub extern "C" fn InitPlugin(id: u64) {
            let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("rustplugin.log").unwrap());
            info!("Hello from a Rust plugin!");

            bakkesmod::bakkesmod_init(id);
            #name();

            info!("finished initialization");
        }
    };

    tokens.into()
}