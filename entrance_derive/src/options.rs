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

        let short = get_short_attribute(&name_value_attrs)?;
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

fn get_short_attribute(
    name_value_attrs: &[syn::MetaNameValue],
) -> Result<Option<char>, &'static str> {
    Ok(match get_single_attribute("short", name_value_attrs) {
        Some(lit) => {
            if let syn::Lit::Char(c) = lit {
                Some(c.value())
            } else {
                return Err("Invalid usage of `short` attribute: expected a char");
            }
        }
        None => None,
    })
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
    let options: Vec<OptionItem> = fields
        .named
        .iter()
        .map(|field| field.try_into().unwrap())
        .collect();

    let names = options.iter().map(|option| &option.name);
    let declare_lines = quote! {
        #(
            let mut #names = false;
        )*
    };

    let long_option_arms = options.iter().map(|option| long_option_arm(&option.name));
    let short_option_arms = options.iter().filter_map(|option| {
        let name = &option.name;
        let short = option.short?;
        Some(quote! {
            #short => #name = true,
        })
    });
    let parse_lines = quote! {
        match option {
            entrance::OptionItem::Long(option) => {
                match option.as_str() {
                    #(
                        #long_option_arms
                    )*
                    _ => {
                        return Err(entrance::ErrorKind::InvalidOption.into());
                    }
                }
            }
            entrance::OptionItem::Short(o) => {
                match o {
                    #(
                        #short_option_arms
                    )*
                    _ => {
                        return Err(entrance::ErrorKind::InvalidOption.into());
                    }
                }
            }
        }
    };

    let names = options.iter().map(|option| &option.name);
    let parse_impl = quote! {
        fn parse<I: std::iter::Iterator<Item = entrance::OptionItem>>(
            mut options: I,
        ) -> std::result::Result<Self, entrance::Error> {
            #declare_lines

            for option in options {
                #parse_lines
            }

            Ok(Self {
                #(
                    #names,
                )*
            })
        }
    };

    let num_options = options.len();
    let names = options.iter().map(|option| &option.name);
    let descriptions = options.iter().map(|option| &option.description);
    let shorts = options.iter().map(|option| option_to_tokens(option.short));
    let opts_impl = quote! {
        fn spec() -> &'static [entrance::Opt] {
            static OPTS: [entrance::Opt; #num_options] = [
                #(
                    entrance::Opt {
                        long: stringify!(#names),
                        short: #shorts,
                        description: #descriptions,
                    },
                )*
            ];
            &OPTS
        }
    };

    quote! {
        #parse_impl
        #opts_impl
    }
}
