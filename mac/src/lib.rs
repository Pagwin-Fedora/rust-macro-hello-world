extern crate syn;
extern crate quote;

use std::str::FromStr;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn hello_and_goodbye(_attr: TokenStream, tokens:TokenStream)->TokenStream{
    let tmp = tokens.clone();
    let input = parse_macro_input!(tokens as Item);
    if let Item::Fn(mut fun) = input{
        let prepend_stream = TokenStream::from_str(format!(r#"println!("Hello {}");"#,fun.sig.ident.to_string()).as_str()).unwrap();
        let postpend_stream = TokenStream::from_str(format!(r#"println!("Goodbye {}");"#,fun.sig.ident.to_string()).as_str()).unwrap();
        let local = parse_macro_input!(prepend_stream as syn::Stmt);
        fun.block.stmts.insert(0, local);
        let local = parse_macro_input!(postpend_stream as syn::Stmt);
        fun.block.stmts.push(local);
        fun.into_token_stream().into()
    }
    else{
        quote!{
            compile_error!("Can only run on a function");
        }.into()
    }
}
