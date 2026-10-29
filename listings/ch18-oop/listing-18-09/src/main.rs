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

// ANCHOR: here
use arayuz::{Dugme, Ekran};

fn main() {
    let ekran = Ekran {
        bilesenler: vec![
            Box::new(SecimKutusu {
                genislik: 75,
                yukseklik: 10,
                secenekler: vec![
                    String::from("Evet"),
                    String::from("Belki"),
                    String::from("Hayır"),
                ],
            }),
            Box::new(Dugme {
                genislik: 50,
                yukseklik: 10,
                etiket: String::from("Tamam"),
            }),
        ],
    };

    ekran.calistir();
}
// ANCHOR_END: here
