pub trait Ozet {
    fn ozetle(&self) -> String;
}

pub struct HaberMakalesi {
    pub manset: String,
    pub konum: String,
    pub yazar: String,
    pub icerik: String,
}

impl Ozet for HaberMakalesi {
    fn ozetle(&self) -> String {
        format!("{}, {} ({})", self.manset, self.yazar, self.konum)
    }
}

pub struct SosyalGonderi {
    pub kullanici_adi: String,
    pub icerik: String,
    pub yanit: bool,
    pub yeniden_paylasim: bool,
}

impl Ozet for SosyalGonderi {
    fn ozetle(&self) -> String {
        format!("{}: {}", self.kullanici_adi, self.icerik)
    }
}

// ANCHOR: here
fn ozetlenebilir_dondur(degistir: bool) -> impl Ozet {
    if degistir {
        HaberMakalesi {
            manset: String::from(
                "Penguenler Stanley Cup Şampiyonluğunu kazandı!",
            ),
            konum: String::from("Pittsburgh, PA, ABD"),
            yazar: String::from("Iceburgh"),
            icerik: String::from(
                "Pittsburgh Penguenleri bir kez daha NHL'deki en iyi \
                 hokey takımı.",
            ),
        }
    } else {
        SosyalGonderi {
            kullanici_adi: String::from("horse_ebooks"),
            icerik: String::from(
                "elbette, muhtemelen zaten bildiğiniz gibi, insanlar",
            ),
            yanit: false,
            yeniden_paylasim: false,
        }
    }
}
// ANCHOR_END: here
