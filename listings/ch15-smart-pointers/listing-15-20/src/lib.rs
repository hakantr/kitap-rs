pub trait Iletici {
    fn gonder(&self, ileti: &str);
}

pub struct SinirIzleyici<'a, T: Iletici> {
    iletici: &'a T,
    deger: usize,
    en_buyuk: usize,
}

impl<'a, T> SinirIzleyici<'a, T>
where
    T: Iletici,
{
    pub fn yeni(iletici: &'a T, en_buyuk: usize) -> SinirIzleyici<'a, T> {
        SinirIzleyici {
            iletici,
            deger: 0,
            en_buyuk,
        }
    }

    pub fn deger_ata(&mut self, deger: usize) {
        self.deger = deger;

        let en_buyugun_yuzdesi = self.deger as f64 / self.en_buyuk as f64;

        if en_buyugun_yuzdesi >= 1.0 {
            self.iletici.gonder("Hata: Kotanizi astiniz!");
        } else if en_buyugun_yuzdesi >= 0.9 {
            self.iletici
                .gonder("Acil uyari: Kotanizin %90'ini gectiniz!");
        } else if en_buyugun_yuzdesi >= 0.75 {
            self.iletici.gonder("Uyari: Kotanizin %75'ini gectiniz!");
        }
    }
}
