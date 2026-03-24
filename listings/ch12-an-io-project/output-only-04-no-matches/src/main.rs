use std::env;
use std::process;

use minigrep::Yapilandirma;

fn main() {
    let argumanlar: Vec<String> = env::args().collect();

    let yapilandirma = Yapilandirma::olustur(&argumanlar).unwrap_or_else(|hata| {
        println!("Argümanları ayrıştırırken problem oluştu: {hata}");
        process::exit(1);
    });

    if let Err(e) = minigrep::calistir(yapilandirma) {
        println!("Uygulama hatası: {e}");
        process::exit(1);
    }
}
