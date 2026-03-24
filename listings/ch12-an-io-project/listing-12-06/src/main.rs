use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let argumanlar: Vec<String> = env::args().collect();

    let yapilandirma = yapilandirma_ayristir(&argumanlar);

    println!("Aranan: {}", yapilandirma.sorgu);
    println!("Dosya: {}", yapilandirma.dosya_yolu);

    let icerik =
        fs::read_to_string(yapilandirma.dosya_yolu).expect("Dosya okunamadı");

    // --snip--
    // ANCHOR_END: here

    println!("Metin içeriği:\n{icerik}");
    // ANCHOR: here
}

struct Yapilandirma {
    sorgu: String,
    dosya_yolu: String,
}

fn yapilandirma_ayristir(argumanlar: &[String]) -> Yapilandirma {
    let sorgu = argumanlar[1].clone();
    let dosya_yolu = argumanlar[2].clone();

    Yapilandirma { sorgu, dosya_yolu }
}
// ANCHOR_END: here
