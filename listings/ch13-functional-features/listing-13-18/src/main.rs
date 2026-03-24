use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{ara, buyuk_kucuk_harf_duyarsiz_ara};

// ANCHOR: here
fn main() {
    let config = Yapilandirma::olustur(env::argumanlar()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
    // ANCHOR_END: here

    if let Err(e) = calistir(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    // ANCHOR: here
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
