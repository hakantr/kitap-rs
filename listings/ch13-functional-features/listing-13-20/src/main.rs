use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{ara, buyuk_kucuk_harf_duyarsiz_ara};

fn main() {
    let yapilandirma =
        Yapilandirma::olustur(env::args()).unwrap_or_else(|hata| {
            eprintln!("Argümanları ayrıştırırken problem oluştu: {hata}");
            process::exit(1);
        });

    if let Err(e) = calistir(yapilandirma) {
        eprintln!("Uygulama hatası: {e}");
        process::exit(1);
    }
}

pub struct Yapilandirma {
    pub sorgu: String,
    pub dosya_yolu: String,
    pub buyuk_kucuk_harf_yoksay: bool,
}

// ANCHOR: here
impl Yapilandirma {
    fn olustur(
        mut argumanlar: impl Iterator<Item = String>,
    ) -> Result<Yapilandirma, &'static str> {
        argumanlar.next();

        let sorgu = match argumanlar.next() {
            Some(arg) => arg,
            None => return Err("Sorgu metni alınamadı"),
        };

        let dosya_yolu = match argumanlar.next() {
            Some(arg) => arg,
            None => return Err("Dosya yolu alınamadı"),
        };

        let buyuk_kucuk_harf_yoksay = env::var("IGNORE_CASE").is_ok();

        Ok(Yapilandirma {
            sorgu,
            dosya_yolu,
            buyuk_kucuk_harf_yoksay,
        })
    }
}
// ANCHOR_END: here

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
