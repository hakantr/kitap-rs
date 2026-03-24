// ANCHOR: here
pub fn ara<'a>(sorgu: &str, icerik: &'a str) -> Vec<&'a str> {
    icerik
        .lines()
        .filter(|satir| satir.contains(sorgu))
        .collect()
}
// ANCHOR_END: here

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let sorgu = "duct";
        let icerik = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], ara(sorgu, icerik));
    }

    #[test]
    fn case_insensitive() {
        let sorgu = "rUsT";
        let icerik = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            buyuk_kucuk_harf_duyarsiz_ara(sorgu, icerik)
        );
    }
}
