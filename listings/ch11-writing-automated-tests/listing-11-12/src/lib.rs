pub fn iki_ekle(a: u64) -> u64 {
    ic_toplayici(a, 2)
}

fn ic_toplayici(sol: u64, sag: u64) -> u64 {
    sol + sag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ic() {
        let sonuc = ic_toplayici(2, 2);
        assert_eq!(sonuc, 4);
    }
}
