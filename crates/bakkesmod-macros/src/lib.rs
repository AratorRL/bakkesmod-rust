extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn plugin_init(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let parsed_input: syn::ItemFn = syn::parse_macro_input!(input as syn::ItemFn);
    let name = parsed_input.clone().sig.ident;

    let tokens = quote! {
        #parsed_input

        #[no_mangle]
        pub extern "C" fn InitPlugin(id: u64) {
            bakkesmod_init(id);
            #name();
        }

        #[no_mangle]
        pub extern "C" fn ExitPlugin() {
            bakkesmod_exit();
        }
    };

    tokens.into()
}