extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Rect)]
pub fn derive_rectangle(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let gen = quote! {
        impl Rect for #name {
            fn area(&self) -> f64 {
                self.length * self.width
            }
        }
    };
    gen.into()
}
