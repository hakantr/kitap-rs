// ANCHOR: here
fn ilk_kelime(metin: &String) -> usize {
    // ANCHOR: as_bytes
    let baytlar = metin.as_bytes();
    // ANCHOR_END: as_bytes

    // ANCHOR: iter
    for (i, &oge) in baytlar.iter().enumerate() {
        // ANCHOR_END: iter
        // ANCHOR: inside_for
        if oge == b' ' {
            return i;
        }
    }

    metin.len()
    // ANCHOR_END: inside_for
}
// ANCHOR_END: here

fn main() {}
