mod restoran_on_kisim {
    mod karsilama {
        fn bekleme_listesine_ekle() {}
    }
}

pub fn restoranda_yemek_ye() {
    // Mutlak (absolute) yol
    crate::restoran_on_kisim::karsilama::bekleme_listesine_ekle();

    // Göreceli (relative) yol
    restoran_on_kisim::karsilama::bekleme_listesine_ekle();
}
