extern crate proc_macro;

use proc_macro::TokenStream;
mod derived;
mod inject;

/// Marks a struct as a Shellder component
#[proc_macro_derive(Hooks)]
pub fn derive_hooks(input: TokenStream) -> TokenStream {
    derived::derive_hooks_impl(input)
}

#[proc_macro_derive(Inject, attributes(component))]
pub fn inject(item: TokenStream) -> TokenStream {
    inject::inject_derive(item)
}

