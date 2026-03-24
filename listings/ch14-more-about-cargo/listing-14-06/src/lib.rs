//! # Sanat
//!
//! Sanatsal kavramlari modellemek icin bir kutuphane.

pub use self::turler::BirincilRenk;
pub use self::turler::IkincilRenk;
pub use self::yardimcilar::karistir;

pub mod turler {
    /// RYB renk modeline gore birincil renkler.
    pub enum BirincilRenk {
        Kirmizi,
        Sari,
        Mavi,
    }

    /// RYB renk modeline gore ikincil renkler.
    pub enum IkincilRenk {
        Turuncu,
        Yesil,
        Mor,
    }
}

pub mod yardimcilar {
    use crate::turler::*;

    /// Iki birincil rengi esit miktarda birlestirerek
    /// bir ikincil renk olusturur.
    pub fn karistir(renk1: BirincilRenk, renk2: BirincilRenk) -> IkincilRenk {
        let _ = (renk1, renk2);
        IkincilRenk::Turuncu
    }
}
