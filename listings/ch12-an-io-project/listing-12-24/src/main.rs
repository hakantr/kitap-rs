use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{ara, buyuk_kucuk_harf_duyarsiz_ara};

// ANCHOR: here
fn main() {
    let argumanlar: Vec<String> = env::args().collect();

    let yapilandirma =
        Yapilandirma::olustur(&argumanlar).unwrap_or_else(|hata| {
            eprintln!("Argümanları ayrıştırırken problem oluştu: {hata}");
            process::exit(1);
        });

    if let Err(e) = calistir(yapilandirma) {
        eprintln!("Uygulama hatası: {e}");
        process::exit(1);
    }
}
// ANCHOR_END: here

pub struct Yapilandirma {
    pub sorgu: String,
    pub dosya_yolu: String,
    pub buyuk_kucuk_harf_yoksay: bool,
}

impl Yapilandirma {
    fn olustur(argumanlar: &[String]) -> Result<Yapilandirma, &'static str> {
        if argumanlar.len() < 3 {
            return Err("yeterli argüman yok");
        }

        let sorgu = argumanlar[1].clone();
        let dosya_yolu = argumanlar[2].clone();

        let buyuk_kucuk_harf_yoksay = env::var("IGNORE_CASE").is_ok();

        Ok(Yapilandirma {
            sorgu,
            dosya_yolu,
            buyuk_kucuk_harf_yoksay,
        })
    }
}

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
