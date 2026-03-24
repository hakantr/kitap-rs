// ANCHOR: here
fn ilk_kelime(metin: &String) -> &str {
    let baytlar = metin.as_bytes();

    for (i, &oge) in baytlar.iter().enumerate() {
        if oge == b' ' {
            return &metin[0..i];
        }
    }

    &metin[..]
}
// ANCHOR_END: here

fn main() {}
