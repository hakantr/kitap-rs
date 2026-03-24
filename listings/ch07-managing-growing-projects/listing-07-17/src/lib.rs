mod restoran_on_kisim {
    pub mod karsilama {
        pub fn bekleme_listesine_ekle() {}
    }
}

pub use crate::restoran_on_kisim::karsilama;

pub fn restoranda_yemek_ye() {
    karsilama::bekleme_listesine_ekle();
}
