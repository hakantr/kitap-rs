use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("Sayıyı tahmin et!");

    let gizli_sayi = rand::rng().random_range(1..=100);

    loop {
        println!("Lütfen tahmininizi girin.");

        let mut tahmin = String::new();

        io::stdin()
            .read_line(&mut tahmin)
            .expect("Satır okunamadı");

        let tahmin: u32 = match tahmin.trim().parse() {
            Ok(sayi) => sayi,
            Err(_) => continue,
        };

        println!("Tahmininiz: {tahmin}");

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
