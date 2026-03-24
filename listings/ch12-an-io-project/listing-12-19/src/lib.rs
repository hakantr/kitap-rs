// ANCHOR: here
// ANCHOR: ch13
pub fn ara<'a>(sorgu: &str, icerik: &'a str) -> Vec<&'a str> {
    let mut sonuclar = Vec::new();

    for satir in icerik.lines() {
        if satir.contains(sorgu) {
            sonuclar.push(satir);
        }
    }

    sonuclar
}
// ANCHOR_END: ch13
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tek_sonuc() {
        let sorgu = "güven";
        let icerik = "\
Güven:
güvenli, hızlı, üretken.
Üçünü de seç.";

        assert_eq!(vec!["güvenli, hızlı, üretken."], ara(sorgu, icerik));
    }
}
