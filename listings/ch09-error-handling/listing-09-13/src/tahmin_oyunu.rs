pub struct Tahmin {
    deger: i32,
}

impl Tahmin {
    pub fn new(deger: i32) -> Tahmin {
        if deger < 1 || deger > 100 {
            panic!(
                "Tahmin değeri 1 ile 100 arasında olmalıdır, {deger} alındı."
            );
        }

        Tahmin { deger }
    }

    pub fn deger(&self) -> i32 {
        self.deger
    }
}
