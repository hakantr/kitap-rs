// ANCHOR: here
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // --snip--
    // ANCHOR_END: here
    println!("Sayıyı tahmin et!");

    let gizli_sayi = rand::thread_rng().gen_range(1..=100);

    println!("Gizli sayı: {gizli_sayi}");

    println!("Lütfen tahmininizi girin.");

    let mut tahmin = String::new();

    io::stdin()
        .read_line(&mut tahmin)
        .expect("Satır okunamadı");
    // ANCHOR: here

    println!("Tahmininiz: {tahmin}");

    match tahmin.cmp(&gizli_sayi) {
        Ordering::Less => println!("Çok küçük!"),
        Ordering::Greater => println!("Çok büyük!"),
        Ordering::Equal => println!("Kazandınız!"),
    }
}
// ANCHOR_END: here
