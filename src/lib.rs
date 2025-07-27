// Copyright (c) 2025 Saugata Kundu - kundusaugata576@gmail.com
// Licensed under the Apache-2.0 License

extern crate proc_macro;
use proc_macro::TokenStream;
mod hook;
mod inject;
mod application;

/// Marks a struct as a Shellder component
#[proc_macro_derive(Hooks)]
pub fn derive_hooks(input: TokenStream) -> TokenStream {
    hook::derive_hooks_impl(input)
}

#[proc_macro_derive(Inject, attributes(component))]
pub fn inject(item: TokenStream) -> TokenStream {
    inject::inject_derive(item)
}

#[proc_macro_derive(App, attributes(component))]
pub fn app(item: TokenStream) -> TokenStream {
    application::app_impl(item)
}
