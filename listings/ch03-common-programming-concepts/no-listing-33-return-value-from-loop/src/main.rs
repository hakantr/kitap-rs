fn main() {
    let mut sayac = 0;

    let sonuc = loop {
        sayac += 1;

        if sayac == 10 {
            break sayac * 2;
        }
    };

    println!("sonuc değeri: {sonuc}");
}
