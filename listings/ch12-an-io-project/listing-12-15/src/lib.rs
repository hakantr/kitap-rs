pub fn ara<'a>(sorgu: &str, icerik: &'a str) -> Vec<&'a str> {
    unimplemented!();
}

// ANCHOR: here
// --snip--

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
// ANCHOR_END: here
