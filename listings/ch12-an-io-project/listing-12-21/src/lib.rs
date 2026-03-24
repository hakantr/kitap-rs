pub fn ara<'a>(sorgu: &str, icerik: &'a str) -> Vec<&'a str> {
    let mut sonuclar = Vec::new();

    for satir in icerik.lines() {
        if satir.contains(sorgu) {
            sonuclar.push(satir);
        }
    }

    sonuclar
}

// ANCHOR: here
pub fn buyuk_kucuk_harf_duyarsiz_ara<'a>(
    sorgu: &str,
    icerik: &'a str,
) -> Vec<&'a str> {
    let sorgu = sorgu.to_lowercase();
    let mut sonuclar = Vec::new();

    for satir in icerik.lines() {
        if satir.to_lowercase().contains(&sorgu) {
            sonuclar.push(satir);
        }
    }

    sonuclar
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buyuk_kucuk_harf_duyarli() {
        let sorgu = "güven";
        let icerik = "\
Güven:
güvenli, hızlı, üretken.
Üçünü de seç.
Güven kolay kazanılmaz.";

        assert_eq!(vec!["güvenli, hızlı, üretken."], ara(sorgu, icerik));
    }

    #[test]
    fn buyuk_kucuk_harf_duyarsiz() {
        let sorgu = "güven";
        let icerik = "\
Güven:
önce sağlamlık gelir.
Planlı çalış, sakin ol.
güven kazanılır.";

        assert_eq!(
            vec!["Güven:", "güven kazanılır."],
            buyuk_kucuk_harf_duyarsiz_ara(sorgu, icerik)
        );
    }
}
