use std::ops::Deref;

impl<T> Deref for BenimKutu<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct BenimKutu<T>(T);

impl<T> BenimKutu<T> {
    fn yeni(x: T) -> BenimKutu<T> {
        BenimKutu(x)
    }
}

fn merhaba(isim: &str) {
    println!("Merhaba, {isim}!");
}

// ANCHOR: here
fn main() {
    let kutu = BenimKutu::yeni(String::from("Rust"));
    merhaba(&(*kutu)[..]);
}
// ANCHOR_END: here
