use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn impl_options(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Not supported for any type except Struct"),
    };

    match fields {
        syn::Fields::Named(fields) => impl_for_named_fields(name, fields),
        _ => panic!("Not supported for any Struct without named fields"),
    }
}

fn long_option_arm(option: &syn::Ident) -> impl quote::ToTokens {
    quote!(
        stringify!(#option) => #option = true,
    )
}

fn impl_for_named_fields(name: &syn::Ident, fields: &syn::FieldsNamed) -> TokenStream {
    let names: Vec<_> = fields
        .named
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();

    let names_for_declare = names.iter();
    let declare_lines = quote! {
        #(
            let mut #names_for_declare = false;
        )*
    };

    let option_arms = names.iter().map(|opt| long_option_arm(opt));
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
        } else {
            break;
        }
        args.next(); // Consume an element
    };

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

    let num_options = fields.named.len();
    let options = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
    let opts_impl = quote! {
        fn spec() -> &'static [entrance::Opt] {
            static OPTS: [entrance::Opt; #num_options] = [
                #(
                    entrance::Opt {
                        long: stringify!(#options),
                        short: None,
                        description: "",
                    },
                )*
            ];
            &OPTS
        }
    };

    let gen = quote! {
        impl entrance::Options for #name {
            #consume_impl
            #opts_impl
        }
    };
    gen.into()
}
