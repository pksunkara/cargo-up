use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

mod impls;
mod utils;

/// Generates the upgrader impl
#[proc_macro_attribute]
#[proc_macro_error]
pub fn upgrader(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    impls::upgrader(parse_macro_input!(attr), &input).into()
}
