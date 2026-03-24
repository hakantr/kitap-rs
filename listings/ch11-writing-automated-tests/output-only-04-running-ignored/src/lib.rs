pub fn topla(sol: u64, sag: u64) -> u64 {
    sol + sag
}

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calisiyor() {
        let sonuc = topla(2, 2);
        assert_eq!(sonuc, 4);
    }

    #[test]
    #[ignore]
    fn pahali_test() {
        // çalışması bir saat süren kod
    }
}
// ANCHOR_END: here
