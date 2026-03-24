// ANCHOR: here
fn ilk_kelime(s: &str) -> &str {
    let baytlar = s.as_bytes();

    for (i, &oge) in baytlar.iter().enumerate() {
        if oge == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// ANCHOR_END: here

fn main() {
    let benim_dizgim = String::from("merhaba dünya");

    // ilk_kelime `String`lerin dilimleri üzerinde çalışır
    let kelime = ilk_kelime(&benim_dizgim[..]);

    let benim_dizgi_sabitim = "merhaba dünya";

    // ilk_kelime string sabitlerinin dilimleri üzerinde çalışır
    let kelime = ilk_kelime(&benim_dizgi_sabitim[..]);

    // String sabitleri *zaten* string dilimi olduğundan,
    // bu dilim sözdizimi olmadan da çalışır!
    let kelime = ilk_kelime(benim_dizgi_sabitim);
}
