// Copyright (c) 2025 Saugata Kundu - kundusaugata576@gmail.com
// Licensed under the Apache-2.0 License

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Fields, GenericArgument, PathArguments, Type};

fn strip_arc(ty: &Type) -> &Type {
    if let Type::Path(type_path) = ty {
        if let Some(seg) = type_path.path.segments.last() {
            if seg.ident == "Arc" {
                if let PathArguments::AngleBracketed(ref args) = seg.arguments {
                    if let Some(GenericArgument::Type(ref inner_ty)) = args.args.first() {
                        return inner_ty;
                    }
                }
            }
        }
    }
    ty
}

fn is_component(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("component"))
}

pub fn app_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match input.data {
        syn::Data::Struct(ref data_struct) => match &data_struct.fields {
            Fields::Named(named) => &named.named,
            _ => panic!("Inject only supports structs with named fields"),
        },
        _ => panic!("Inject only supports structs"),
    };
    let mut inject_args = Vec::new();
    let mut constructor_fields = Vec::new();
    let mut register_feilds = Vec::new();

    for f in fields {
        let fname = &f.ident;

        if is_component(&f.attrs) {
            let fty = strip_arc(&f.ty);
            constructor_fields.push(quote! {
                #fname: container.resolve::<#fty>().expect("Inject Error")
            });
            register_feilds.push(quote! {
                #fty
            });
        } else {
            let fty = &f.ty;
            inject_args.push(quote! {
                #fname: #fty
            });
            constructor_fields.push(quote! {
                #fname: #fname
            });
        }
    };

    let gen = quote! {
        impl Lifecycle for #name {
            fn run_hooks(&self){
                self.startup();
                self.run();
                self.cleanup();
            }
        }
        impl #name {
            fn inject(container: &Container, #(#inject_args),*) -> Self {
                Self {
                    #(#constructor_fields,)*
                }
            }
        }

        fn main() {
            let container = Container::new();
            #(container.register(#register_feilds::new());)*
            let app = #name::inject(&container);
            app.run_hooks();
        }
    };

    TokenStream::from(gen)
}

