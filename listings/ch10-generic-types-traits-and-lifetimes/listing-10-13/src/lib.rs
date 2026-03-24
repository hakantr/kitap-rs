pub trait Ozet {
    fn ozetle(&self) -> String;
}

// ANCHOR: here
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
// ANCHOR_END: here
