struct BenimKutu<T>(T);

impl<T> BenimKutu<T> {
    fn yeni(x: T) -> BenimKutu<T> {
        BenimKutu(x)
    }
}

// ANCHOR: here
fn main() {
    let sayi = 5;
    let kutu = BenimKutu::yeni(sayi);

    assert_eq!(5, sayi);
    assert_eq!(5, *kutu);
}
// ANCHOR_END: here
