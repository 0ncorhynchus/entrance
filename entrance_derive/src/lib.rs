#![recursion_limit = "128"]
extern crate proc_macro;

mod arguments;
mod options;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Arguments, attributes(description, variable_argument))]
pub fn args_derive(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as arguments::ArgumentsInput);
    input.gen()
}

#[proc_macro_derive(Options, attributes(description, short))]
pub fn options_derive(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as options::OptionsInput);
    input.gen()
}

fn extract_name_values(attrs: &[syn::Attribute]) -> Vec<syn::MetaNameValue> {
    attrs
        .iter()
        .filter_map(syn::Attribute::interpret_meta)
        .filter_map(|meta| {
            if let syn::Meta::NameValue(name_value) = meta {
                Some(name_value)
            } else {
                None
            }
        })
        .collect()
}

fn get_single_attribute<'a>(
    name: &'static str,
    attrs: &'a [syn::MetaNameValue],
) -> Option<&'a syn::Lit> {
    let extracted: Vec<_> = attrs
        .iter()
        .filter(|attr| attr.ident == name)
        .map(|attr| &attr.lit)
        .collect();
    if extracted.len() > 1 {
        panic!(format!("Invalid duplicated attribute `{}`", name));
    }
    extracted.into_iter().next()
}

fn get_description(name_values: &[syn::MetaNameValue]) -> String {
    if let Some(lit) = get_single_attribute("description", name_values) {
        if let syn::Lit::Str(string) = lit {
            string.value()
        } else {
            panic!("Invalid usage of the `description` attribute");
        }
    } else {
        String::new()
    }
}
