use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields};
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

pub fn inject_impl(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let name = &input.ident;
    let fields = match &input.fields {
        Fields::Named(named) => &named.named,
        _ => {
            return syn::Error::new_spanned(
                &input,
                "inject only supports structs with named fields"
            )
            .to_compile_error()
            .into();
        }
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| {
        let original_ty = &f.ty;
        let ty = strip_arc(original_ty);
        eprintln!("Original: {}, Stripped: {}", quote!(#original_ty), quote!(#ty));
        ty
    }).collect();


    let expanded = quote! {
        #input

        impl Injection for #name {
            fn inject(container: &Container) -> Self {
                Self {
                    #(
                        #field_names: container.resolve::<#field_types>().expect("Inject Error:"),
                    )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
