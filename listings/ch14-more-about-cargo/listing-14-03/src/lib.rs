// ANCHOR: here
//! # Sanat
//!
//! Sanatsal kavramlari modellemek icin bir kutuphane.

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
        // --snip--
        // ANCHOR_END: here
        let _ = (renk1, renk2);
        unimplemented!();
        // ANCHOR: here
    }
}
// ANCHOR_END: here
