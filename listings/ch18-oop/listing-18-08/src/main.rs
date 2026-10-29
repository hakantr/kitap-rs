// ANCHOR: here
use arayuz::Ciz;

struct SecimKutusu {
    genislik: u32,
    yukseklik: u32,
    secenekler: Vec<String>,
}

impl Ciz for SecimKutusu {
    fn ciz(&self) {
        // bir seçim kutusunu gerçekten çizmek için kod
    }
}
// ANCHOR_END: here

fn main() {}
