pub fn bir_ekle(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calisir() {
        assert_eq!(3, bir_ekle(2));
    }
}
