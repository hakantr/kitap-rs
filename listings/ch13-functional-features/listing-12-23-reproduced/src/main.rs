use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{ara, buyuk_kucuk_harf_duyarsiz_ara};

fn main() {
    let argumanlar: Vec<String> = env::argumanlar().collect();

    let config = Yapilandirma::olustur(&argumanlar).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = calistir(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

pub struct Yapilandirma {
    pub sorgu: String,
    pub dosya_yolu: String,
    pub buyuk_kucuk_harf_yoksay: bool,
}

// ANCHOR: ch13
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
// ANCHOR_END: ch13

fn calistir(config: Yapilandirma) -> Result<(), Box<dyn Error>> {
    let icerik = fs::read_to_string(config.dosya_yolu)?;

    let sonuclar = if config.buyuk_kucuk_harf_yoksay {
        buyuk_kucuk_harf_duyarsiz_ara(&config.sorgu, &icerik)
    } else {
        ara(&config.sorgu, &icerik)
    };

    for satir in sonuclar {
        println!("{satir}");
    }

    Ok(())
}
