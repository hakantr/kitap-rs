pub fn iki_ekle(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iki_ve_iki_ekle() {
        assert_eq!(4, iki_ekle(2));
    }

    #[test]
    fn uc_ve_iki_ekle() {
        assert_eq!(5, iki_ekle(3));
    }

    #[test]
    fn yuz() {
        assert_eq!(102, iki_ekle(100));
    }
}
