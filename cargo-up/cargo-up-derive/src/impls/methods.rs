use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};

pub struct Methods;

impl Parse for Methods {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Methods {})
    }
}

pub fn rename_struct_methods(input: Methods) -> TokenStream {
    quote! {}
}
