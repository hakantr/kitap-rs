// ANCHOR: here
use arayuz::Ciz;

struct SecimKutusu {
    genislik: u32,
    yukseklik: u32,
    secenekler: Vec<String>,
}

impl Ciz for SecimKutusu {
    fn ciz(&self) {
        // bir secim kutusunu gercekten cizmek icin kod
    }
}
// ANCHOR_END: here

fn main() {}
