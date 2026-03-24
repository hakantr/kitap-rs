// ANCHOR: here
pub trait Ozet {
    fn ozetle(&self) -> String {
        String::from("(Devamını oku...)")
    }
}
// ANCHOR_END: here

pub struct HaberMakalesi {
    pub manset: String,
    pub konum: String,
    pub yazar: String,
    pub icerik: String,
}

impl Ozet for HaberMakalesi {}

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
