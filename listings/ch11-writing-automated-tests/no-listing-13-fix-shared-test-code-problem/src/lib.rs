pub fn iki_ekle(a: usize) -> usize {
    ic_toplayici(a, 2)
}

fn ic_toplayici(sol: usize, sag: usize) -> usize {
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
