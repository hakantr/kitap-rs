use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let argumanlar: Vec<String> = env::args().collect();

    let yapilandirma = Yapilandirma::new(&argumanlar);
    // ANCHOR_END: here

    println!("Aranan: {}", yapilandirma.sorgu);
    println!("Dosya: {}", yapilandirma.dosya_yolu);

    let icerik =
        fs::read_to_string(yapilandirma.dosya_yolu).expect("Dosya okunamadı");

    println!("Metin içeriği:\n{icerik}");
    // ANCHOR: here

    // --snip--
}

// --snip--

// ANCHOR_END: here
struct Yapilandirma {
    sorgu: String,
    dosya_yolu: String,
}

// ANCHOR: here
impl Yapilandirma {
    fn new(argumanlar: &[String]) -> Yapilandirma {
        let sorgu = argumanlar[1].clone();
        let dosya_yolu = argumanlar[2].clone();

        Yapilandirma { sorgu, dosya_yolu }
    }
}
// ANCHOR_END: here
