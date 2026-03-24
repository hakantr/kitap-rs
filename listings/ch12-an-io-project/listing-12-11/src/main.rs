use std::env;
use std::fs;
use std::process;

// ANCHOR: here
fn main() {
    // --snip--

    // ANCHOR_END: here
    let argumanlar: Vec<String> = env::args().collect();

    let yapilandirma =
        Yapilandirma::olustur(&argumanlar).unwrap_or_else(|hata| {
            println!("Argümanları ayrıştırırken problem oluştu: {hata}");
            process::exit(1);
        });

    // ANCHOR: here
    println!("Aranan: {}", yapilandirma.sorgu);
    println!("Dosya: {}", yapilandirma.dosya_yolu);

    calistir(yapilandirma);
}

fn calistir(yapilandirma: Yapilandirma) {
    let icerik =
        fs::read_to_string(yapilandirma.dosya_yolu).expect("Dosya okunamadı");

    println!("Metin içeriği:\n{icerik}");
}

// --snip--
// ANCHOR_END: here

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
