use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(MerhabaMakro)]
pub fn merhaba_makro_derive(input: TokenStream) -> TokenStream {
    // Üzerinde işlem yapabileceğimiz bir sözdizimi ağacı olarak
    // Rust kodunun temsilini oluştur.
    let ast = syn::parse(input).unwrap();

    // Trait uygulamasını oluştur.
    impl_merhaba_makro(&ast)
}
