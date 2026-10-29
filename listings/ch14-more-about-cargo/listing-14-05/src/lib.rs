// ANCHOR: here
//! # Sanat
//!
//! Sanatsal kavramları modellemek için bir kütüphane.

pub use self::turler::BirincilRenk;
pub use self::turler::IkincilRenk;
pub use self::yardimcilar::karistir;

pub mod turler {
    // --snip--
    // ANCHOR_END: here
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
    // ANCHOR: here
}

pub mod yardimcilar {
    // --snip--
    // ANCHOR_END: here
    use crate::turler::*;

    /// İki birincil rengi eşit miktarda birleştirerek
    /// bir ikincil renk oluşturur.
    pub fn karistir(renk1: BirincilRenk, renk2: BirincilRenk) -> IkincilRenk {
        let _ = (renk1, renk2);
        IkincilRenk::Turuncu
    }
    // ANCHOR: here
}
// ANCHOR_END: here
