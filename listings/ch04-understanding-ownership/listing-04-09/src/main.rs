// ANCHOR: here
fn ilk_kelime(metin: &str) -> &str {
    // ANCHOR_END: here
    let baytlar = metin.as_bytes();

    for (i, &oge) in baytlar.iter().enumerate() {
        if oge == b' ' {
            return &metin[0..i];
        }
    }

    &metin[..]
}

// ANCHOR: usage
fn main() {
    let benim_metnim = String::from("merhaba dünya");

    // `ilk_kelime` `String`lerin dilimleri (kısmi veya tam) üzerinde çalışır.
    let kelime = ilk_kelime(&benim_metnim[0..7]);
    let kelime = ilk_kelime(&benim_metnim[..]);
    // `ilk_kelime` ayrıca, `String`lerin tam dilimlerine eşdeğer olan
    // `String` referansları üzerinde de çalışır.
    let kelime = ilk_kelime(&benim_metnim);

    let sabit_metnim = "merhaba dünya";

    // `ilk_kelime` sabit metinlerin dilimleri (kısmi veya tam) üzerinde çalışır.
    let kelime = ilk_kelime(&sabit_metnim[0..7]);
    let kelime = ilk_kelime(&sabit_metnim[..]);

    // Sabit metinler zaten metin dilimleri (string slices) olduğu için,
    // bu da dilim sözdizimi olmadan çalışır!
    let kelime = ilk_kelime(sabit_metnim);
}
// ANCHOR_END: usage
