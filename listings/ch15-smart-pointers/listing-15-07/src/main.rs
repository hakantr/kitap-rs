fn main() {
    let sayi = 5;
    let kutu = Box::new(sayi);

    assert_eq!(5, sayi);
    assert_eq!(5, *kutu);
}
