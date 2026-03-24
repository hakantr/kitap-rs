mod restoran_arka_kisim {
    pub enum Meze {
        Corba,
        Salata,
    }
}

pub fn restoranda_yemek_ye() {
    let siparis1 = restoran_arka_kisim::Meze::Corba;
    let siparis2 = restoran_arka_kisim::Meze::Salata;
}
