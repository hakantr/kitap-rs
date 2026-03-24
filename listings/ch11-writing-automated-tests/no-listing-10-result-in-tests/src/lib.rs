pub fn topla(sol: u64, sag: u64) -> u64 {
    sol + sag
}

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calisiyor() -> Result<(), String> {
        let sonuc = topla(2, 2);

        if sonuc == 4 {
            Ok(())
        } else {
            Err(String::from("iki artı iki dört etmez"))
        }
    }
}
// ANCHOR_END: here
