use crate::{extract_name_values, get_description, get_single_attribute};
use proc_macro::TokenStream;
use quote::quote;
use std::convert::{TryFrom, TryInto};

pub fn impl_options(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Not supported for any type except Struct"),
    };

    let body = match fields {
        syn::Fields::Named(fields) => impl_for_named_fields(fields),
        _ => panic!("Not supported for any Struct without named fields"),
    };

    let gen = quote! {
        impl entrance::Options for #name {
            #body
        }
    };
    gen.into()
}

struct OptionItem<'a> {
    name: &'a syn::Ident,
    short: Option<char>,
    description: String,
}

impl<'a> TryFrom<&'a syn::Field> for OptionItem<'a> {
    type Error = &'static str;

    fn try_from(field: &'a syn::Field) -> Result<Self, Self::Error> {
        let name = field
            .ident
            .as_ref()
            .ok_or("The tuple structure is not available.")?;
        let name_value_attrs = extract_name_values(&field.attrs);
        let short = get_short_attribute(&name_value_attrs);
        let description = get_description(&name_value_attrs);
        Ok(Self {
            name,
            short,
            description,
        })
    }
}

fn long_option_arm(option: &syn::Ident) -> impl quote::ToTokens {
    quote!(
        stringify!(#option) => #option = true,
    )
}

fn get_short_attribute(name_values: &[syn::MetaNameValue]) -> Option<char> {
    if let syn::Lit::Char(c) = get_single_attribute("short", name_values)? {
        Some(c.value())
    } else {
        panic!("Invalid usage of `short` attribute: expected a char");
    }
}

fn option_to_tokens<T: quote::ToTokens>(x: Option<T>) -> impl quote::ToTokens {
    match x {
        Some(c) => quote! {
            Some(#c)
        },
        None => quote! {
            None
        },
    }
}

fn impl_for_named_fields(fields: &syn::FieldsNamed) -> impl quote::ToTokens {
    let items: Vec<OptionItem> = fields
        .named
        .iter()
        .map(|field| field.try_into().unwrap())
        .collect();

    let names_for_declare = items.iter().map(|item| &item.name);
    let declare_lines = quote! {
        #(
            let mut #names_for_declare = false;
        )*
    };

    let option_arms = items.iter().map(|item| long_option_arm(&item.name));
    let short_option_arms = items.iter().filter_map(|option| {
        let name = &option.name;
        let short = option.short?;
        Some(quote! {
            #short => #name = true,
        })
    });
    let parse_lines = quote! {
        if arg.starts_with("--") {
            match &arg[2..] {
                #(
                    #option_arms
                )*
                flag => {
                    return Err(entrance::OptionError::InvalidLongOption(
                        flag.to_string(),
                    ));
                }
            }
        } else if arg.starts_with("-") {
            for c in arg[1..].chars() {
                match c {
                    #(
                        #short_option_arms
                    )*
                    f => {
                        return Err(entrance::OptionError::InvalidShortOption(f));
                    }
                }
            }
        } else {
            break;
        }
        args.next(); // Consume an element
    };

    let names = items.iter().map(|item| &item.name);
    let consume_impl = quote! {
        fn consume<I: std::iter::Iterator<Item = std::string::String>>(
            args: &mut std::iter::Peekable<I>,
        ) -> std::result::Result<Self, entrance::OptionError> {
            #declare_lines

            while let Some(arg) = args.peek() {
                #parse_lines
            }

            Ok(Self {
                #(
                    #names,
                )*
            })
        }
    };

    let num_options = items.len();
    let options = items.iter().map(|option| &option.name);
    let descriptions = items.iter().map(|option| &option.description);
    let shorts = items.iter().map(|option| option_to_tokens(option.short));
    let opts_impl = quote! {
        fn spec() -> &'static [entrance::Opt] {
            static OPTS: [entrance::Opt; #num_options] = [
                #(
                    entrance::Opt {
                        long: stringify!(#options),
                        short: #shorts,
                        description: #descriptions,
                    },
                )*
            ];
            &OPTS
        }
    };

    quote! {
        #consume_impl
        #opts_impl
    }
}
