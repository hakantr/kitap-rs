// ANCHOR: all
fn main() {
    let genislik1 = 30;
    let yukseklik1 = 50;

    println!(
        "Dikdörtgenin alanı {} kare pikseldir.",
        alan(genislik1, yukseklik1)
    );
}

// ANCHOR: here
fn alan(genislik: u32, yukseklik: u32) -> u32 {
    // ANCHOR_END: here
    genislik * yukseklik
}
// ANCHOR_END: all
