/// Marks a struct as a Shellder component
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input struct
    let input = parse_macro_input!(item as ItemStruct);

    // Get the struct name
    let name = &input.ident;

    // Generate an impl block that marks this as a Registerable component
    let expanded = quote! {
        #input
        
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Hooks)]
pub fn derive_hooks(input: TokenStream) -> TokenStream {
    // Parse the input into a syntax tree
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    // Extract the struct name
    let name = &ast.ident;

    // Generate the impl Hooks
    let expanded = quote! {
        impl #name {
            pub fn run_hooks(&self){
                self.startup();
                self.run();
                self.cleanup();
            }
        }
    };

    TokenStream::from(expanded)
}