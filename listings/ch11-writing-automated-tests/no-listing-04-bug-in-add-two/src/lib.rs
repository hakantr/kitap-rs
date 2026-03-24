// ANCHOR: here
pub fn iki_ekle(a: u64) -> u64 {
    a + 3
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iki_ekliyor() {
        let sonuc = iki_ekle(2);
        assert_eq!(sonuc, 4);
    }
}
