pub struct OrtalamaliKoleksiyon {
    liste: Vec<i32>,
    ortalama: f64,
}

// ANCHOR: here
impl OrtalamaliKoleksiyon {
    pub fn ekle(&mut self, deger: i32) {
        self.liste.push(deger);
        self.ortalamayi_guncelle();
    }

    pub fn cikar(&mut self) -> Option<i32> {
        let sonuc = self.liste.pop();
        match sonuc {
            Some(deger) => {
                self.ortalamayi_guncelle();
                Some(deger)
            }
            None => None,
        }
    }

    pub fn ortalama(&self) -> f64 {
        self.ortalama
    }

    fn ortalamayi_guncelle(&mut self) {
        let toplam: i32 = self.liste.iter().sum();
        self.ortalama = toplam as f64 / self.liste.len() as f64;
    }
}
// ANCHOR_END: here
