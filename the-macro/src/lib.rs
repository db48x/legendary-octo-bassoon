#![feature(proc_macro)]

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro::quote;

#[proc_macro_attribute]
pub fn lisp_fn(_attr_ts: TokenStream, _fn_ts: TokenStream) -> TokenStream {
    quote! {
        fn the_value() -> u8 {
            ::std::u8::MAX
        }
    }
}
