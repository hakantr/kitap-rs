// ANCHOR: here
pub trait Ozet {
    fn yazari_ozetle(&self) -> String;

    fn ozetle(&self) -> String {
        format!(
            "({} yazarından daha fazlasını okuyun...)",
            self.yazari_ozetle()
        )
    }
}
// ANCHOR_END: here

pub struct SosyalGonderi {
    pub kullanici_adi: String,
    pub icerik: String,
    pub yanit: bool,
    pub yeniden_paylasim: bool,
}

// ANCHOR: impl
impl Ozet for SosyalGonderi {
    fn yazari_ozetle(&self) -> String {
        format!("@{}", self.kullanici_adi)
    }
}
// ANCHOR_END: impl
