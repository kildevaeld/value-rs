use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Typed)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl Typed for #ident {

        }
    };
    output.into()
}
