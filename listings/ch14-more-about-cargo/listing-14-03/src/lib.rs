// ANCHOR: here
//! # Sanat
//!
//! Sanatsal kavramları modellemek için bir kütüphane.

pub mod turler {
    /// RYB renk modeline göre birincil renkler.
    pub enum BirincilRenk {
        Kirmizi,
        Sari,
        Mavi,
    }

    /// RYB renk modeline göre ikincil renkler.
    pub enum IkincilRenk {
        Turuncu,
        Yesil,
        Mor,
    }
}

pub mod yardimcilar {
    use crate::turler::*;

    /// İki birincil rengi eşit miktarda birleştirerek
    /// bir ikincil renk oluşturur.
    pub fn karistir(renk1: BirincilRenk, renk2: BirincilRenk) -> IkincilRenk {
        // --snip--
        // ANCHOR_END: here
        let _ = (renk1, renk2);
        unimplemented!();
        // ANCHOR: here
    }
}
// ANCHOR_END: here
