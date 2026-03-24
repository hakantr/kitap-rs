fn main() {
    // ANCHOR: here
    enum ElektronikTabloHucresi {
        Tamsayi(i32),
        Ondalikli(f64),
        Metin(String),
    }

    let satir = vec![
        ElektronikTabloHucresi::Tamsayi(3),
        ElektronikTabloHucresi::Metin(String::from("mavi")),
        ElektronikTabloHucresi::Ondalikli(10.12),
    ];
    // ANCHOR_END: here
}
