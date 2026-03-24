// ANCHOR: here
mod restoran_on_kisim {
    pub mod karsilama {
        pub fn bekleme_listesine_ekle() {}
    }
}

// -- snip --
// ANCHOR_END: here
pub fn restoranda_yemek_ye() {
    // Mutlak (absolute) yol
    crate::restoran_on_kisim::karsilama::bekleme_listesine_ekle();

    // Göreceli (relative) yol
    restoran_on_kisim::karsilama::bekleme_listesine_ekle();
}
