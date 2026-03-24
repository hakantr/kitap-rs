use std::env;
use std::fs;
// ANCHOR: here
use std::process;

fn main() {
    let argumanlar: Vec<String> = env::args().collect();

    let yapilandirma =
        Yapilandirma::olustur(&argumanlar).unwrap_or_else(|hata| {
            println!("Argümanları ayrıştırırken problem oluştu: {hata}");
            process::exit(1);
        });

    // --snip--
    // ANCHOR_END: here

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
    fn olustur(argumanlar: &[String]) -> Result<Yapilandirma, &'static str> {
        if argumanlar.len() < 3 {
            return Err("yeterli argüman yok");
        }

        let sorgu = argumanlar[1].clone();
        let dosya_yolu = argumanlar[2].clone();

        Ok(Yapilandirma { sorgu, dosya_yolu })
    }
}
