use proc_macro::TokenStream;

extern crate proc_macro;

#[proc_macro_attribute]
pub fn st_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{attr}");

    item
}
