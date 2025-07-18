extern crate proc_macro;

use proc_macro::TokenStream;
mod derived;
mod attribute;

/// Marks a struct as a Shellder component
#[proc_macro_derive(Hooks)]
pub fn derive_hooks(input: TokenStream) -> TokenStream {
    derived::derive_hooks_impl(input)
}

#[proc_macro_attribute]
pub fn inject(_attr: TokenStream, item: TokenStream) -> TokenStream {
    attribute::inject_impl(item)
}

