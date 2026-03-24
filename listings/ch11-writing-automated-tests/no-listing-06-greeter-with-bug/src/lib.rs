// ANCHOR: here
pub fn selamlama(isim: &str) -> String {
    String::from("Merhaba!")
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selamlama_isim_iceriyor() {
        let sonuc = selamlama("Carol");
        assert!(sonuc.contains("Carol"));
    }
}
