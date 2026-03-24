use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Lütfen bir dizi indeksi girin.");

    let mut indeks = String::new();

    io::stdin()
        .read_line(&mut indeks)
        .expect("Satırı okuyamadı!");

    let indeks: usize = indeks
        .trim()
        .parse()
        .expect("Girilen indeks bir sayi değildi!");

    let element = a[indeks];

    println!("{indeks} indeksindeki elemanın değeri: {element}");
}
