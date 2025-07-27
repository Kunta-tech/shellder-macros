// Copyright (c) 2025 Saugata Kundu - kundusaugata576@gmail.com
// Licensed under the Apache-2.0 License

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


pub fn derive_hooks_impl(input: TokenStream) -> TokenStream {
    // Parse the input into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Extract the struct name
    let name = &ast.ident;

    // Generate the impl Hooks
    let expanded = quote! {
        impl Hookable for #name {
            fn run_hooks(&self){
                self.startup();
                self.run();
                self.cleanup();
            }
        }
    };

    TokenStream::from(expanded)
}