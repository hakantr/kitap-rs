mod restoran_on_kisim {
    mod karsilama {
        fn bekleme_listesine_ekle() {}
    }
}

pub fn restoranda_yemek_ye() {
    // Mutlak yol
    crate::restoran_on_kisim::karsilama::bekleme_listesine_ekle();

    // Göreli yol
    restoran_on_kisim::karsilama::bekleme_listesine_ekle();
}
