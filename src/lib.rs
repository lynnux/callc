extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

// https://rust.programmingpedia.net/en/knowledge-base/56718336/rust-function-name--caller--or-any-other-context-inside-macro
#[proc_macro_attribute]
pub fn stdcall_win(_: TokenStream, func: TokenStream) -> TokenStream {
    if cfg!(windows){
	let mut func = parse_macro_input!(func as ItemFn);
	if let Some(abi) = &mut func.sig.abi{
	    let x = quote::quote!("stdcall");
	    abi.name = Some(syn::parse(x.into()).unwrap())
	}
	let caller = quote!{
            #func
	};
	caller.into() 
    }
    else{
	func
    }
}
