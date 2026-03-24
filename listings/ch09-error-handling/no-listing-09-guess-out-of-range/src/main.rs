use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Sayıyı tahmin et!");

    let gizli_sayi = rand::thread_rng().gen_range(1..=100);

    // ANCHOR: here
    loop {
        // --snip--

        // ANCHOR_END: here
        println!("Lütfen tahmininizi girin.");

        let mut tahmin = String::new();

        io::stdin().read_line(&mut tahmin).expect("Satır okunamadı");

        // ANCHOR: here
        let tahmin: i32 = match tahmin.trim().parse() {
            Ok(sayi) => sayi,
            Err(_) => continue,
        };

        if tahmin < 1 || tahmin > 100 {
            println!("Gizli sayı 1 ile 100 arasında olacak.");
            continue;
        }

        match tahmin.cmp(&gizli_sayi) {
            // --snip--
            // ANCHOR_END: here
            Ordering::Less => println!("Çok küçük!"),
            Ordering::Greater => println!("Çok büyük!"),
            Ordering::Equal => {
                println!("Kazandınız!");
                break;
            }
        }
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
