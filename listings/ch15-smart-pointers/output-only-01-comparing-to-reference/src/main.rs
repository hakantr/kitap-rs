fn main() {
    let sayi = 5;
    let referans = &sayi;

    assert_eq!(5, sayi);
    assert_eq!(5, referans);
}
