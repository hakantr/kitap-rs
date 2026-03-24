// ANCHOR: here
pub fn ara<'a>(sorgu: &str, icerik: &'a str) -> Vec<&'a str> {
    for satir in icerik.lines() {
        if satir.contains(sorgu) {
            // satir ile bir şeyler yapın
        }
    }
}
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
