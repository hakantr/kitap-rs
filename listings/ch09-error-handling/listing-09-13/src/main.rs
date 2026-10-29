use std::cmp::Ordering;
use std::io;
use tahmin_oyunu::Tahmin;

use rand::prelude::*;

mod tahmin_oyunu;

fn main() {
    println!("Sayıyı tahmin et!");

    let gizli_sayi = rand::rng().random_range(1..=100);

    loop {
        println!("Lütfen tahmininizi girin.");

        let mut tahmin = String::new();

        io::stdin()
            .read_line(&mut tahmin)
            .expect("Satır okunamadı");

        let tahmin: i32 = match tahmin.trim().parse() {
            Ok(sayi) => sayi,
            Err(_) => continue,
        };

        let tahmin = Tahmin::new(tahmin);

        match tahmin.deger().cmp(&gizli_sayi) {
            Ordering::Less => println!("Çok küçük!"),
            Ordering::Greater => println!("Çok büyük!"),
            Ordering::Equal => {
                println!("Kazandınız!");
                break;
            }
        }
    }
}
