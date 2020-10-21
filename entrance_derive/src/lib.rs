#![recursion_limit = "128"]
extern crate proc_macro;

mod arguments;
mod extend_syn;
mod options;

use crate::extend_syn::*;
use proc_macro::TokenStream;
use std::convert::TryFrom;
use syn::parse_macro_input;

#[proc_macro_derive(Arguments, attributes(entrance))]
pub fn args_derive(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as arguments::ArgumentsInput);
    input.gen()
}

#[proc_macro_derive(Options, attributes(entrance))]
pub fn options_derive(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as options::OptionsInput);
    input.gen()
}

enum Attribute {
    Description(String), // description
    Variadic,            // variable_argument
    Short(char),         // short
    Informative,         // informative
}

impl TryFrom<&syn::Meta> for Attribute {
    type Error = ();
    fn try_from(meta: &syn::Meta) -> Result<Self, Self::Error> {
        match meta.path().get_ident().unwrap().to_string().as_str() {
            "description" => {
                let desc = meta.name_value().ok_or(())?.lit.str().ok_or(())?;
                Ok(Attribute::Description(desc))
            }
            "variable_argument" => {
                meta.ident().ok_or(())?;
                Ok(Attribute::Variadic)
            }
            "short" => {
                let short = meta.name_value().ok_or(())?.lit.char().ok_or(())?;
                Ok(Attribute::Short(short))
            }
            "informative" => {
                meta.ident().ok_or_else(|| ())?;
                Ok(Attribute::Informative)
            }
            _ => Err(()),
        }
    }
}

fn extract_attributes(attrs: &[syn::Attribute]) -> Vec<(syn::Meta, Attribute)> {
    attrs
        .iter()
        .filter_map(|attr| attr.parse_meta().ok())
        .filter_map(|meta| {
            if let syn::Meta::List(list) = meta {
                Some(list.nested)
            } else {
                None
            }
        })
        .flatten()
        .map(|nested| {
            if let syn::NestedMeta::Meta(meta) = nested {
                let attr = Attribute::try_from(&meta).unwrap();
                return (meta, attr);
            }
            panic!("Invalid attribute is given: not syn::NestedMeta::Meta");
        })
        .collect()
}
