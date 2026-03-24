use rand::Rng;
// ANCHOR: here
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
// ANCHOR_END: here

fn main() {
    println!("Sayıyı tahmin et!");

    let gizli_sayi = rand::thread_rng().gen_range(1..=100);

    println!("Gizli sayı: {gizli_sayi}");

    println!("Lütfen tahmininizi girin.");

    let mut tahmin = String::new();

    io::stdin().read_line(&mut tahmin).expect("Satır okunamadı");

    println!("Tahmininiz: {tahmin}");

    match tahmin.cmp(&gizli_sayi.to_string()) {
        Ordering::Less => println!("Çok küçük!"),
        Ordering::Greater => println!("Çok büyük!"),
        Ordering::Equal => println!("Kazandınız!"),
    }
}
