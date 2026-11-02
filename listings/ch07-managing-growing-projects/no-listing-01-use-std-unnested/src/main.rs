// ANCHOR: here
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
// ANCHOR_END: here

use rand::prelude::*;

fn main() {
    println!("Sayıyı tahmin et!");

    let gizli_sayi = rand::rng().random_range(1..=100);

    println!("Gizli sayı: {gizli_sayi}");

    println!("Lütfen tahmininizi girin.");

    let mut tahmin = String::new();

    io::stdin()
        .read_line(&mut tahmin)
        .expect("Satır okunamadı");

    println!("Tahmininiz: {tahmin}");

    let tahmin: u32 = tahmin.trim().parse().expect("Lütfen bir sayı girin!");

    match tahmin.cmp(&gizli_sayi) {
        Ordering::Less => println!("Çok küçük!"),
        Ordering::Greater => println!("Çok büyük!"),
        Ordering::Equal => println!("Kazandınız!"),
    }
}
