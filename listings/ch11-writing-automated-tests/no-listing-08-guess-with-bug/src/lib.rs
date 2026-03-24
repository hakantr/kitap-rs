pub struct Tahmin {
    deger: i32,
}

// ANCHOR: here
// --snip--
impl Tahmin {
    pub fn new(deger: i32) -> Tahmin {
        if deger < 1 {
            panic!(
                "Tahmin değeri 1 ile 100 arasında olmalıdır, {deger} alındı."
            );
        }

        Tahmin { deger }
    }
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn yuzden_buyuk() {
        Tahmin::new(200);
    }
}
