use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::parse_macro_input;

mod impls;

// Making proc_macros ready for 1.45

#[proc_macro]
#[proc_macro_error]
pub fn rename_struct_methods(input: TokenStream) -> TokenStream {
    impls::rename_struct_methods(parse_macro_input!(input)).into()
}
