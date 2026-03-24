struct OnemliAlinti<'a> {
    bolum: &'a str,
}

fn main() {
    let roman = String::from("Bana İsmail deyin. Birkaç yıl önce...");
    let ilk_cumle = roman.split('.').next().unwrap();
    let i = OnemliAlinti { bolum: ilk_cumle };
}
