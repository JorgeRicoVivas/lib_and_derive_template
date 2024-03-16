use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};


#[proc_macro_derive(MyDerive, attributes(MyDeriveAttributeA, MyDeriveAttributeB))]
pub fn my_proc_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::new()
}

#[proc_macro_attribute]
pub fn my_proc_macro_attribute(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    item
}

#[proc_macro]
pub fn my_proc_macro(item: TokenStream) -> TokenStream {
    TokenStream::new()
}
