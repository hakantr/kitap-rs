use std::env;
use std::fs;

fn main() {
    let argumanlar: Vec<String> = env::args().collect();

    let yapilandirma = Yapilandirma::new(&argumanlar);

    println!("Aranan: {}", yapilandirma.sorgu);
    println!("Dosya: {}", yapilandirma.dosya_yolu);

    let icerik =
        fs::read_to_string(yapilandirma.dosya_yolu).expect("Dosya okunamadı");

    println!("Metin içeriği:\n{icerik}");
}

struct Yapilandirma {
    sorgu: String,
    dosya_yolu: String,
}

impl Yapilandirma {
    // ANCHOR: here
    // --snip--
    fn new(argumanlar: &[String]) -> Yapilandirma {
        if argumanlar.len() < 3 {
            panic!("yeterli argüman yok");
        }
        // --snip--
        // ANCHOR_END: here

        let sorgu = argumanlar[1].clone();
        let dosya_yolu = argumanlar[2].clone();

        Yapilandirma { sorgu, dosya_yolu }
    }
}
