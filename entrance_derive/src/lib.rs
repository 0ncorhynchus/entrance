extern crate proc_macro;

mod options;
mod arguments;

use proc_macro::TokenStream;

#[proc_macro_derive(Arguments)]
pub fn args_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    arguments::impl_arguments(&ast)
}

#[proc_macro_derive(Options)]
pub fn optionss_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    options::impl_options(&ast)
}
