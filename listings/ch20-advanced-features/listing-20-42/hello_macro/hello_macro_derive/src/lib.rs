use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(MerhabaMakro)]
pub fn merhaba_makro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_merhaba_makro(&ast)
}

// ANCHOR: here
fn impl_merhaba_makro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl MerhabaMakro for #name {
            fn merhaba_makro() {
                println!("Merhaba, Makro! Benim adım {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}
// ANCHOR_END: here
