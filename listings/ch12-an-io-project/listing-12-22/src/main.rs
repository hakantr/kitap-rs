use std::env;
use std::error::Error;
use std::fs;
use std::process;

// ANCHOR: there
use minigrep::{ara, buyuk_kucuk_harf_duyarsiz_ara};

// --snip--

// ANCHOR_END: there

fn main() {
    let argumanlar: Vec<String> = env::args().collect();

    let yapilandirma =
        Yapilandirma::olustur(&argumanlar).unwrap_or_else(|hata| {
            println!("Argümanları ayrıştırırken problem oluştu: {hata}");
            process::exit(1);
        });

    if let Err(e) = calistir(yapilandirma) {
        println!("Uygulama hatası: {e}");
        process::exit(1);
    }
}

// ANCHOR: here
pub struct Yapilandirma {
    pub sorgu: String,
    pub dosya_yolu: String,
    pub buyuk_kucuk_harf_yoksay: bool,
}
// ANCHOR_END: here

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

// ANCHOR: there
fn calistir(yapilandirma: Yapilandirma) -> Result<(), Box<dyn Error>> {
    let icerik = fs::read_to_string(yapilandirma.dosya_yolu)?;

    let sonuclar = if yapilandirma.buyuk_kucuk_harf_yoksay {
        buyuk_kucuk_harf_duyarsiz_ara(&yapilandirma.sorgu, &icerik)
    } else {
        ara(&yapilandirma.sorgu, &icerik)
    };

    for satir in sonuclar {
        println!("{satir}");
    }

    Ok(())
}
// ANCHOR_END: there
