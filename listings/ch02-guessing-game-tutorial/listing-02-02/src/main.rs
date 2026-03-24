use std::io;

fn main() {
    println!("Sayıyı tahmin et!");

    println!("Lütfen tahmininizi girin.");

    let mut tahmin = String::new();

    io::stdin()
        .read_line(&mut tahmin)
        .expect("Satır okunamadı");

    println!("Tahmininiz: {tahmin}");
}
