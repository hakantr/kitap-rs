use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Sayıyı tahmin et!");

    let gizli_sayi = rand::rng().random_range(1..=100);

    println!("Gizli sayı: {gizli_sayi}");

    loop {
        println!("Lütfen tahmininizi girin.");

        let mut tahmin = String::new();

        // ANCHOR: here
        // --snip--

        io::stdin()
            .read_line(&mut tahmin)
            .expect("Satır okunamadı");

        // ANCHOR: ch19
        let tahmin: u32 = match tahmin.trim().parse() {
            Ok(sayi) => sayi,
            Err(_) => continue,
        };
        // ANCHOR_END: ch19

        println!("Tahmininiz: {tahmin}");

        // --snip--
        // ANCHOR_END: here

        match tahmin.cmp(&gizli_sayi) {
            Ordering::Less => println!("Çok küçük!"),
            Ordering::Greater => println!("Çok büyük!"),
            Ordering::Equal => {
                println!("Kazandınız!");
                break;
            }
        }
    }
}
