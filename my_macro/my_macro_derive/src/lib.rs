extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

#[proc_macro_derive(Serialize)]
pub fn my_macro_serialize(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let res = impl_macro_serialize(&ast);
    dbg!(res.to_string());
    res
}

fn serialize_encode(data: &Data) -> proc_macro2::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Unnamed(ref fields) => {
                let rec = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let self_ = quote!(self);
                    let res = quote!(res);
                    quote_spanned! {f.span() =>
                    #res.append(&mut #self_.#i.serialize());
                    }
                });

                quote!( #( #rec )* )
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn impl_macro_serialize(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let encoded = serialize_encode(&ast.data);

    let gen = quote! {
    impl Serialize for #name {
        fn serialize(&self) -> Vec<u8> {
        let mut res = vec![];
    #encoded
        res
        }
    }
    };
    gen.into()
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
