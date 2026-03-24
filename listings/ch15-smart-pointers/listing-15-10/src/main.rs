// ANCHOR: here
use std::ops::Deref;

impl<T> Deref for BenimKutu<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// ANCHOR_END: here

struct BenimKutu<T>(T);

impl<T> BenimKutu<T> {
    fn yeni(x: T) -> BenimKutu<T> {
        BenimKutu(x)
    }
}

fn main() {
    let sayi = 5;
    let kutu = BenimKutu::yeni(sayi);

    assert_eq!(5, sayi);
    assert_eq!(5, *kutu);
}
