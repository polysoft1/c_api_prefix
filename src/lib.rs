extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn, Abi, Token, LitStr};

#[proc_macro_attribute]
pub fn c_api_prefix(attr: TokenStream, func: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(func as ItemFn);
    
    let prefix = attr.to_string();
    let name = input.sig.ident.to_string();

    input.sig.ident = get_name(prefix, name);

    input.sig.abi = abi_setting();
    let tokens = input.to_token_stream();
    TokenStream::from(
        quote!{
            #[no_mangle]
            #tokens
        }
    )
}

fn abi_setting() -> Option<Abi> {
    Some(Abi {
        extern_token: Token![extern](Span::call_site()),
        name: Some(LitStr::new("C", Span::call_site()))
    })
}

fn get_name(prefix: String, name: String) -> Ident {
    match prefix.as_str() {
        "" => Ident::new(name.as_str(), Span::call_site()),
        prefix => Ident::new(
            format!("{}_{}", prefix, name).as_str(),
            Span::call_site()
        )
    }
}