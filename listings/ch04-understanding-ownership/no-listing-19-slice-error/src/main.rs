fn ilk_kelime(metin: &String) -> &str {
    let baytlar = metin.as_bytes();

    for (i, &oge) in baytlar.iter().enumerate() {
        if oge == b' ' {
            return &metin[0..i];
        }
    }

    &metin[..]
}

// ANCHOR: here
fn main() {
    let mut metin = String::from("merhaba dünya");

    let kelime = ilk_kelime(&metin);

    metin.clear(); // hata!

    println!("ilk kelime: {kelime}");
}
// ANCHOR_END: here
