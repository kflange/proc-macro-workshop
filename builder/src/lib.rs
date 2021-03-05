use proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    //   let ast = syn::parse(input)
    let _ast: syn::DeriveInput = syn::parse(input).unwrap_or_else(|e| panic!("{}", e));

    let gen = quote! {};
    gen.into()
}
