fn main() {
    // ANCHOR: here
    let max_ayari = Some(3u8);
    match max_ayari {
        Some(max) => println!("Maksimum değer {max} olarak ayarlandı"),
        _ => (),
    }
    // ANCHOR_END: here
}
