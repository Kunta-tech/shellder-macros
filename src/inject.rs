use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Attribute};
use syn::{Type, PathArguments, GenericArgument};

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


pub fn inject_derive(input: TokenStream) -> TokenStream {
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

    for f in fields {
        let fname = &f.ident;

        if is_component(&f.attrs) {
            let fty = strip_arc(&f.ty);
            constructor_fields.push(quote! {
                #fname: container.resolve::<#fty>().expect("Inject Error")
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

    let expanded = quote! {
        impl #name {
            fn inject(container: &Container, #(#inject_args),*) -> Self {
                Self {
                    #(#constructor_fields,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
