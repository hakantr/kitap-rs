mod restoran_arka_kisim {
    pub struct Kahvalti {
        pub tost: String,
        mevsim_meyvesi: String,
    }

    impl Kahvalti {
        pub fn yaz(tost: &str) -> Kahvalti {
            Kahvalti {
                tost: String::from(tost),
                mevsim_meyvesi: String::from("şeftali"),
            }
        }
    }
}

pub fn restoranda_yemek_ye() {
    // Çavdar tostu ile bir yaz kahvaltısı sipariş edin.
    let mut ogun = restoran_arka_kisim::Kahvalti::yaz("Çavdar");
    // Hangi ekmeği istediğimiz konusundaki fikrimizi değiştirelim.
    ogun.tost = String::from("Buğday");
    println!("Lütfen {} tost istiyorum", ogun.tost);

    // Aşağıdaki satırın yorumunu kaldırırsak derlenmeyecektir; öğünle birlikte gelen
    // mevsim meyvesini görmemize veya değiştirmemize izin verilmez.
    // ogun.mevsim_meyvesi = String::from("yaban mersini");
}
