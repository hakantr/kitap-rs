pub struct Tahmin {
    deger: i32,
}

impl Tahmin {
    pub fn new(deger: i32) -> Tahmin {
        // ANCHOR: here
        if deger < 1 {
            panic!(
                "Tahmin değeri 100'den küçük veya 100'e eşit olmalıdır, {deger} alındı."
            );
        } else if deger > 100 {
            panic!(
                "Tahmin değeri 1'den büyük veya 1'e eşit olmalıdır, {deger} alındı."
            );
        }
        // ANCHOR_END: here

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
