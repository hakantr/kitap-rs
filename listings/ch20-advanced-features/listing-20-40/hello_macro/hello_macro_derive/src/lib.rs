use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(MerhabaMakro)]
pub fn merhaba_makro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_merhaba_makro(&ast)
}
