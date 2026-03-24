pub fn selamlama(isim: &str) -> String {
    format!("Merhaba {isim}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selamlama_isim_iceriyor() {
        let sonuc = selamlama("Carol");
        assert!(sonuc.contains("Carol"));
    }
}
