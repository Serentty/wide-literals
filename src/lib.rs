extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse, LitStr};

#[proc_macro]
pub fn w(input: TokenStream) -> TokenStream {
    let string_literal: LitStr = match parse(input) {
        Ok(literal) => literal,
        Err(_) => panic!("Something other than a string literal was provided.")
    };
    let string = string_literal.value();
    let mut output = String::from("&[");
    for (i, code_unit) in string.encode_utf16().chain(std::iter::once(0)).enumerate() {
        output.push_str(&format!("0x{:X}", code_unit));
        if i == 0 {
            output.push_str("u16");
        }
        output.push(',');
    }
    output.push_str("] as *const u16");
    output.parse().unwrap()
}