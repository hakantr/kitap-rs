use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let argumanlar: Vec<String> = env::args().collect();

    let (sorgu, dosya_yolu) = yapilandirma_ayristir(&argumanlar);

    // --snip--
    // ANCHOR_END: here

    println!("Aranan: {sorgu}");
    println!("Dosya: {dosya_yolu}");

    let icerik = fs::read_to_string(dosya_yolu).expect("Dosya okunamadı");

    println!("Metin içeriği:\n{icerik}");
    // ANCHOR: here
}

fn yapilandirma_ayristir(argumanlar: &[String]) -> (&str, &str) {
    let sorgu = &argumanlar[1];
    let dosya_yolu = &argumanlar[2];

    (sorgu, dosya_yolu)
}
// ANCHOR_END: here
