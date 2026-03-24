// ANCHOR: here
use std::env;
use std::fs;

fn main() {
    // --snip--
    // ANCHOR_END: here
    let argumanlar: Vec<String> = env::args().collect();

    let sorgu = &argumanlar[1];
    let dosya_yolu = &argumanlar[2];

    println!("Aranan: {sorgu}");
    // ANCHOR: here
    println!("Dosya: {dosya_yolu}");

    let icerik = fs::read_to_string(dosya_yolu).expect("Dosya okunamadı");

    println!("Metin içeriği:\n{icerik}");
}
// ANCHOR_END: here
