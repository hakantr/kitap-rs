struct OnemliAlinti<'a> {
    bolum: &'a str,
}

// ANCHOR: 1st
impl<'a> OnemliAlinti<'a> {
    fn seviye(&self) -> i32 {
        3
    }
}
// ANCHOR_END: 1st

// ANCHOR: 3rd
impl<'a> OnemliAlinti<'a> {
    fn duyur_ve_bolumu_dondur(&self, duyuru: &str) -> &str {
        println!("Lütfen dikkat: {duyuru}");
        self.bolum
    }
}
// ANCHOR_END: 3rd

fn main() {
    let roman = String::from("Bana İsmail deyin. Birkaç yıl önce...");
    let ilk_cumle = roman.split('.').next().unwrap();
    let i = OnemliAlinti { bolum: ilk_cumle };
}
