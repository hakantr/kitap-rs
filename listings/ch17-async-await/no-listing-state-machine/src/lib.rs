extern crate trpl; // required for mdbook test

// ANCHOR: enum
enum SayfaBasligiFuture<'a> {
    Baslangic { url: &'a str },
    GetBeklemeNoktasi { url: &'a str },
    MetinBeklemeNoktasi { yanit: trpl::Response },
}
// ANCHOR_END: enum
