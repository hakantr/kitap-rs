pub struct Tahmin {
    deger: i32,
}

// ANCHOR: here
// --snip--

impl Tahmin {
    pub fn new(deger: i32) -> Tahmin {
        if deger < 1 {
            panic!(
                "Tahmin değeri 1'den büyük veya 1'e eşit olmalıdır, {deger} alındı."
            );
        } else if deger > 100 {
            panic!(
                "Tahmin değeri 100'den küçük veya 100'e eşit olmalıdır, {deger} alındı."
            );
        }

        Tahmin { deger }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "100'den küçük veya 100'e eşit")]
    fn yuzden_buyuk() {
        Tahmin::new(200);
    }
}
// ANCHOR_END: here
