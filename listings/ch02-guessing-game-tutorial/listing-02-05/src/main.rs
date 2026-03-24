use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Sayıyı tahmin et!");

    let gizli_sayi = rand::thread_rng().gen_range(1..=100);

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
            Ok(num) => num,
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
