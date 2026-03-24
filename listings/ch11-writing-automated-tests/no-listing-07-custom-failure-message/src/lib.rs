pub fn selamlama(isim: &str) -> String {
    String::from("Merhaba!")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ANCHOR: here
    #[test]
    fn selamlama_isim_iceriyor() {
        let sonuc = selamlama("Carol");
        assert!(
            sonuc.contains("Carol"),
            "Selamlama isim içermiyordu, değer `{sonuc}` idi"
        );
    }
    // ANCHOR_END: here
}
