extern crate proc_macro;

mod arguments;
mod options;

use proc_macro::TokenStream;

#[proc_macro_derive(Arguments, attributes(description))]
pub fn args_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    arguments::impl_arguments(&ast)
}

#[proc_macro_derive(VariableArgument, attributes(description))]
pub fn var_arg_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    arguments::impl_variable_argument(&ast)
}

#[proc_macro_derive(Options, attributes(description, short))]
pub fn options_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    options::impl_options(&ast)
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
