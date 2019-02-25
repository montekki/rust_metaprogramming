extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

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

fn serialize_encode(_data: &Data) -> proc_macro2::TokenStream {
    quote!(println!("wip");)
}

fn impl_macro_serialize(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let encoded = serialize_encode(&ast.data);

    let gen = quote! {
    impl Serialize for #name {
        fn serialize(&self) -> Vec<u8> {
        let res = vec![];
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
