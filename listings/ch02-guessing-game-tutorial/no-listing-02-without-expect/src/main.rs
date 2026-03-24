use std::io;

fn main() {
    println!("Sayıyı tahmin et!");

    println!("Lütfen tahmininizi girin.");

    let mut tahmin = String::new();

    io::stdin().read_line(&mut tahmin);

    println!("Tahmininiz: {tahmin}");
}
