pub fn topla(sol: u64, sag: u64) -> u64 {
    sol + sag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kesif() {
        let sonuc = topla(2, 2);
        assert_eq!(sonuc, 4);
    }
}
