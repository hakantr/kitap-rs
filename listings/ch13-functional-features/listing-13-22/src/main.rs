use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::ara;

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

fn calistir(config: Yapilandirma) -> Result<(), Box<dyn Error>> {
    let icerik = fs::read_to_string(config.dosya_yolu)?;

    for satir in ara(&config.sorgu, &icerik) {
        println!("{satir}");
    }

    Ok(())
}
