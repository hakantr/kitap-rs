pub fn iki_ekle(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iki_ve_iki_ekle() {
        let sonuc = iki_ekle(2);
        assert_eq!(sonuc, 4);
    }

    #[test]
    fn uc_ve_iki_ekle() {
        let sonuc = iki_ekle(3);
        assert_eq!(sonuc, 5);
    }

    #[test]
    fn yuz() {
        let sonuc = iki_ekle(100);
        assert_eq!(sonuc, 102);
    }
}
